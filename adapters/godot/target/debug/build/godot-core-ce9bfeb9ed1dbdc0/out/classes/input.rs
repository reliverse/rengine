#![doc = "Sidecar module for class [`Input`][crate::classes::Input].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Input` enums](https://docs.godotengine.org/en/stable/classes/class_input.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Input.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`input`][crate::classes::input]: sidecar module with related enum/flag types\n* [`SignalsOfInput`][crate::classes::input::SignalsOfInput]: signal collection\n\n\nSee also [Godot docs for `Input`](https://docs.godotengine.org/en/stable/classes/class_input.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Input {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Input {
        pub fn is_anything_pressed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_anything_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_key_pressed(&self, keycode: crate::global::Key,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::global::Key,);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_key_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_physical_key_pressed(&self, keycode: crate::global::Key,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::global::Key,);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_physical_key_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_key_label_pressed(&self, keycode: crate::global::Key,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::global::Key,);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_key_label_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_mouse_button_pressed(&self, button: crate::global::MouseButton,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::global::MouseButton,);
            let args = (button,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_mouse_button_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_joy_button_pressed(&self, device: i32, button: crate::global::JoyButton,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, crate::global::JoyButton,);
            let args = (device, button,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_joy_button_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn is_action_pressed_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_action_pressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_action_pressed_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_action_pressed(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_pressed_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_pressed_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExIsActionPressed < 'a > {
            ExIsActionPressed::new(self, action,)
        }
        pub(crate) fn is_action_just_pressed_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_action_just_pressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_action_just_pressed_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_action_just_pressed(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_just_pressed_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_just_pressed_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExIsActionJustPressed < 'a > {
            ExIsActionJustPressed::new(self, action,)
        }
        pub(crate) fn is_action_just_released_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_action_just_released", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_action_just_released_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_action_just_released(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_just_released_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_just_released_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExIsActionJustReleased < 'a > {
            ExIsActionJustReleased::new(self, action,)
        }
        pub(crate) fn is_action_just_pressed_by_event_full(&self, action: CowArg < StringName >, event: CowArg < Option < Gd < crate::classes::InputEvent > > >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::InputEvent > > >, bool,);
            let args = (action, event, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_action_just_pressed_by_event", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_action_just_pressed_by_event_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_action_just_pressed_by_event(&self, action: impl AsArg < StringName >, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) -> bool {
            self.is_action_just_pressed_by_event_ex(action, event,) . done()
        }
        #[inline]
        pub fn is_action_just_pressed_by_event_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'a,) -> ExIsActionJustPressedByEvent < 'a > {
            ExIsActionJustPressedByEvent::new(self, action, event,)
        }
        pub(crate) fn is_action_just_released_by_event_full(&self, action: CowArg < StringName >, event: CowArg < Option < Gd < crate::classes::InputEvent > > >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::InputEvent > > >, bool,);
            let args = (action, event, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_action_just_released_by_event", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_action_just_released_by_event_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_action_just_released_by_event(&self, action: impl AsArg < StringName >, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) -> bool {
            self.is_action_just_released_by_event_ex(action, event,) . done()
        }
        #[inline]
        pub fn is_action_just_released_by_event_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'a,) -> ExIsActionJustReleasedByEvent < 'a > {
            ExIsActionJustReleasedByEvent::new(self, action, event,)
        }
        pub(crate) fn get_action_strength_full(&self, action: CowArg < StringName >, exact_match: bool,) -> f32 {
            type CallRet = f32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_action_strength", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_action_strength_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_action_strength(&self, action: impl AsArg < StringName >,) -> f32 {
            self.get_action_strength_ex(action,) . done()
        }
        #[inline]
        pub fn get_action_strength_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExGetActionStrength < 'a > {
            ExGetActionStrength::new(self, action,)
        }
        pub(crate) fn get_action_raw_strength_full(&self, action: CowArg < StringName >, exact_match: bool,) -> f32 {
            type CallRet = f32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_action_raw_strength", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_action_raw_strength_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_action_raw_strength(&self, action: impl AsArg < StringName >,) -> f32 {
            self.get_action_raw_strength_ex(action,) . done()
        }
        #[inline]
        pub fn get_action_raw_strength_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExGetActionRawStrength < 'a > {
            ExGetActionRawStrength::new(self, action,)
        }
        pub fn get_axis(&self, negative_action: impl AsArg < StringName >, positive_action: impl AsArg < StringName >,) -> f32 {
            type CallRet = f32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (negative_action.into_arg(), positive_action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_vector_full(&self, negative_x: CowArg < StringName >, positive_x: CowArg < StringName >, negative_y: CowArg < StringName >, positive_y: CowArg < StringName >, deadzone: f32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >, CowArg < 'a3, StringName >, f32,);
            let args = (negative_x, positive_x, negative_y, positive_y, deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4398usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_vector", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_vector_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_vector(&self, negative_x: impl AsArg < StringName >, positive_x: impl AsArg < StringName >, negative_y: impl AsArg < StringName >, positive_y: impl AsArg < StringName >,) -> Vector2 {
            self.get_vector_ex(negative_x, positive_x, negative_y, positive_y,) . done()
        }
        #[inline]
        pub fn get_vector_ex < 'a > (&'a self, negative_x: impl AsArg < StringName > + 'a, positive_x: impl AsArg < StringName > + 'a, negative_y: impl AsArg < StringName > + 'a, positive_y: impl AsArg < StringName > + 'a,) -> ExGetVector < 'a > {
            ExGetVector::new(self, negative_x, positive_x, negative_y, positive_y,)
        }
        pub(crate) fn add_joy_mapping_full(&mut self, mapping: CowArg < GString >, update_existing: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (mapping, update_existing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4399usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "add_joy_mapping", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_joy_mapping_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_joy_mapping(&mut self, mapping: impl AsArg < GString >,) {
            self.add_joy_mapping_ex(mapping,) . done()
        }
        #[inline]
        pub fn add_joy_mapping_ex < 'a > (&'a mut self, mapping: impl AsArg < GString > + 'a,) -> ExAddJoyMapping < 'a > {
            ExAddJoyMapping::new(self, mapping,)
        }
        pub fn remove_joy_mapping(&mut self, guid: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (guid.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "remove_joy_mapping", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_joy_known(&mut self, device: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_joy_known", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joy_axis(&self, device: i32, axis: crate::global::JoyAxis,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, crate::global::JoyAxis,);
            let args = (device, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_joy_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joy_name(&mut self, device: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_joy_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joy_guid(&self, device: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_joy_guid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joy_info(&self, device: i32,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_joy_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn should_ignore_device(&self, vendor_id: i32, product_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (vendor_id, product_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "should_ignore_device", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_connected_joypads(&mut self,) -> Array < i64 > {
            type CallRet = Array < i64 >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_connected_joypads", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joy_vibration_strength(&mut self, device: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_joy_vibration_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joy_vibration_duration(&mut self, device: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_joy_vibration_duration", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn start_joy_vibration_full(&mut self, device: i32, weak_magnitude: f32, strong_magnitude: f32, duration: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32, f32, f32,);
            let args = (device, weak_magnitude, strong_magnitude, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "start_joy_vibration", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::start_joy_vibration_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn start_joy_vibration(&mut self, device: i32, weak_magnitude: f32, strong_magnitude: f32,) {
            self.start_joy_vibration_ex(device, weak_magnitude, strong_magnitude,) . done()
        }
        #[inline]
        pub fn start_joy_vibration_ex < 'a > (&'a mut self, device: i32, weak_magnitude: f32, strong_magnitude: f32,) -> ExStartJoyVibration < 'a > {
            ExStartJoyVibration::new(self, device, weak_magnitude, strong_magnitude,)
        }
        pub fn stop_joy_vibration(&mut self, device: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "stop_joy_vibration", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn vibrate_handheld_full(&mut self, duration_ms: i32, amplitude: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (duration_ms, amplitude,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "vibrate_handheld", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::vibrate_handheld_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn vibrate_handheld(&mut self,) {
            self.vibrate_handheld_ex() . done()
        }
        #[inline]
        pub fn vibrate_handheld_ex < 'a > (&'a mut self,) -> ExVibrateHandheld < 'a > {
            ExVibrateHandheld::new(self,)
        }
        pub fn get_gravity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accelerometer(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_accelerometer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_magnetometer(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_magnetometer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gyroscope(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_gyroscope", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity(&mut self, value: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accelerometer(&mut self, value: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_accelerometer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_magnetometer(&mut self, value: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_magnetometer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gyroscope(&mut self, value: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_gyroscope", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_last_mouse_velocity(&mut self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_last_mouse_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_last_mouse_screen_velocity(&mut self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_last_mouse_screen_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mouse_button_mask(&self,) -> crate::global::MouseButtonMask {
            type CallRet = crate::global::MouseButtonMask;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_mouse_button_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mouse_mode(&mut self, mode: crate::classes::input::MouseMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::input::MouseMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_mouse_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mouse_mode(&self,) -> crate::classes::input::MouseMode {
            type CallRet = crate::classes::input::MouseMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_mouse_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn warp_mouse(&mut self, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "warp_mouse", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn action_press_full(&mut self, action: CowArg < StringName >, strength: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f32,);
            let args = (action, strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "action_press", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::action_press_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn action_press(&mut self, action: impl AsArg < StringName >,) {
            self.action_press_ex(action,) . done()
        }
        #[inline]
        pub fn action_press_ex < 'a > (&'a mut self, action: impl AsArg < StringName > + 'a,) -> ExActionPress < 'a > {
            ExActionPress::new(self, action,)
        }
        pub fn action_release(&mut self, action: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "action_release", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_default_cursor_shape_full(&mut self, shape: crate::classes::input::CursorShape,) {
            type CallRet = ();
            type CallParams = (crate::classes::input::CursorShape,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_default_cursor_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_default_cursor_shape_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_default_cursor_shape(&mut self,) {
            self.set_default_cursor_shape_ex() . done()
        }
        #[inline]
        pub fn set_default_cursor_shape_ex < 'a > (&'a mut self,) -> ExSetDefaultCursorShape < 'a > {
            ExSetDefaultCursorShape::new(self,)
        }
        pub fn get_current_cursor_shape(&self,) -> crate::classes::input::CursorShape {
            type CallRet = crate::classes::input::CursorShape;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_current_cursor_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_custom_mouse_cursor_full(&mut self, image: CowArg < Option < Gd < crate::classes::Resource > > >, shape: crate::classes::input::CursorShape, hotspot: Vector2,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource > > >, crate::classes::input::CursorShape, Vector2,);
            let args = (image, shape, hotspot,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_custom_mouse_cursor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_custom_mouse_cursor_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_custom_mouse_cursor(&mut self, image: impl AsArg < Option < Gd < crate::classes::Resource >> >,) {
            self.set_custom_mouse_cursor_ex(image,) . done()
        }
        #[inline]
        pub fn set_custom_mouse_cursor_ex < 'a > (&'a mut self, image: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'a,) -> ExSetCustomMouseCursor < 'a > {
            ExSetCustomMouseCursor::new(self, image,)
        }
        pub fn parse_input_event(&mut self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::InputEvent > > >,);
            let args = (event.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4432usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "parse_input_event", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_accumulated_input(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4433usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_use_accumulated_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_accumulated_input(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4434usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_using_accumulated_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn flush_buffered_events(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4435usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "flush_buffered_events", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emulate_mouse_from_touch(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4436usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_emulate_mouse_from_touch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_emulating_mouse_from_touch(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4437usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_emulating_mouse_from_touch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emulate_touch_from_mouse(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4438usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_emulate_touch_from_mouse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_emulating_touch_from_mouse(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4439usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_emulating_touch_from_mouse", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Input {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Input"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Input {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Input {
        
    }
    impl crate::obj::Singleton for Input {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"Input"))
            }
        }
    }
    impl std::ops::Deref for Input {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Input {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Input__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Input` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Input::is_action_pressed_ex`][super::Input::is_action_pressed_ex]."]
#[must_use]
pub struct ExIsActionPressed < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionPressed < 'a > {
    fn new(surround_object: &'a re_export::Input, action: impl AsArg < StringName > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::Input::is_action_pressed_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::is_action_just_pressed_ex`][super::Input::is_action_just_pressed_ex]."]
#[must_use]
pub struct ExIsActionJustPressed < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionJustPressed < 'a > {
    fn new(surround_object: &'a re_export::Input, action: impl AsArg < StringName > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::Input::is_action_just_pressed_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::is_action_just_released_ex`][super::Input::is_action_just_released_ex]."]
#[must_use]
pub struct ExIsActionJustReleased < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionJustReleased < 'a > {
    fn new(surround_object: &'a re_export::Input, action: impl AsArg < StringName > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::Input::is_action_just_released_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::is_action_just_pressed_by_event_ex`][super::Input::is_action_just_pressed_by_event_ex]."]
#[must_use]
pub struct ExIsActionJustPressedByEvent < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, action: CowArg < 'a, StringName >, event: CowArg < 'a, Option < Gd < crate::classes::InputEvent > > >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionJustPressedByEvent < 'a > {
    fn new(surround_object: &'a re_export::Input, action: impl AsArg < StringName > + 'a, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), event: event.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, action, event, exact_match,
        }
        = self;
        re_export::Input::is_action_just_pressed_by_event_full(surround_object, action, event, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::is_action_just_released_by_event_ex`][super::Input::is_action_just_released_by_event_ex]."]
#[must_use]
pub struct ExIsActionJustReleasedByEvent < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, action: CowArg < 'a, StringName >, event: CowArg < 'a, Option < Gd < crate::classes::InputEvent > > >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionJustReleasedByEvent < 'a > {
    fn new(surround_object: &'a re_export::Input, action: impl AsArg < StringName > + 'a, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), event: event.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, action, event, exact_match,
        }
        = self;
        re_export::Input::is_action_just_released_by_event_full(surround_object, action, event, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::get_action_strength_ex`][super::Input::get_action_strength_ex]."]
#[must_use]
pub struct ExGetActionStrength < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetActionStrength < 'a > {
    fn new(surround_object: &'a re_export::Input, action: impl AsArg < StringName > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::Input::get_action_strength_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::get_action_raw_strength_ex`][super::Input::get_action_raw_strength_ex]."]
#[must_use]
pub struct ExGetActionRawStrength < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetActionRawStrength < 'a > {
    fn new(surround_object: &'a re_export::Input, action: impl AsArg < StringName > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::Input::get_action_raw_strength_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::get_vector_ex`][super::Input::get_vector_ex]."]
#[must_use]
pub struct ExGetVector < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, negative_x: CowArg < 'a, StringName >, positive_x: CowArg < 'a, StringName >, negative_y: CowArg < 'a, StringName >, positive_y: CowArg < 'a, StringName >, deadzone: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVector < 'a > {
    fn new(surround_object: &'a re_export::Input, negative_x: impl AsArg < StringName > + 'a, positive_x: impl AsArg < StringName > + 'a, negative_y: impl AsArg < StringName > + 'a, positive_y: impl AsArg < StringName > + 'a,) -> Self {
        let deadzone = - 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, negative_x: negative_x.into_arg(), positive_x: positive_x.into_arg(), negative_y: negative_y.into_arg(), positive_y: positive_y.into_arg(), deadzone: deadzone,
        }
    }
    #[inline]
    pub fn deadzone(self, deadzone: f32) -> Self {
        Self {
            deadzone: deadzone, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        let Self {
            _phantom, surround_object, negative_x, positive_x, negative_y, positive_y, deadzone,
        }
        = self;
        re_export::Input::get_vector_full(surround_object, negative_x, positive_x, negative_y, positive_y, deadzone,)
    }
}
#[doc = "Default-param extender for [`Input::add_joy_mapping_ex`][super::Input::add_joy_mapping_ex]."]
#[must_use]
pub struct ExAddJoyMapping < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Input, mapping: CowArg < 'a, GString >, update_existing: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddJoyMapping < 'a > {
    fn new(surround_object: &'a mut re_export::Input, mapping: impl AsArg < GString > + 'a,) -> Self {
        let update_existing = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mapping: mapping.into_arg(), update_existing: update_existing,
        }
    }
    #[inline]
    pub fn update_existing(self, update_existing: bool) -> Self {
        Self {
            update_existing: update_existing, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, mapping, update_existing,
        }
        = self;
        re_export::Input::add_joy_mapping_full(surround_object, mapping, update_existing,)
    }
}
#[doc = "Default-param extender for [`Input::start_joy_vibration_ex`][super::Input::start_joy_vibration_ex]."]
#[must_use]
pub struct ExStartJoyVibration < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Input, device: i32, weak_magnitude: f32, strong_magnitude: f32, duration: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStartJoyVibration < 'a > {
    fn new(surround_object: &'a mut re_export::Input, device: i32, weak_magnitude: f32, strong_magnitude: f32,) -> Self {
        let duration = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, device: device, weak_magnitude: weak_magnitude, strong_magnitude: strong_magnitude, duration: duration,
        }
    }
    #[inline]
    pub fn duration(self, duration: f32) -> Self {
        Self {
            duration: duration, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, device, weak_magnitude, strong_magnitude, duration,
        }
        = self;
        re_export::Input::start_joy_vibration_full(surround_object, device, weak_magnitude, strong_magnitude, duration,)
    }
}
#[doc = "Default-param extender for [`Input::vibrate_handheld_ex`][super::Input::vibrate_handheld_ex]."]
#[must_use]
pub struct ExVibrateHandheld < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Input, duration_ms: i32, amplitude: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVibrateHandheld < 'a > {
    fn new(surround_object: &'a mut re_export::Input,) -> Self {
        let duration_ms = 500i32;
        let amplitude = - 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, duration_ms: duration_ms, amplitude: amplitude,
        }
    }
    #[inline]
    pub fn duration_ms(self, duration_ms: i32) -> Self {
        Self {
            duration_ms: duration_ms, .. self
        }
    }
    #[inline]
    pub fn amplitude(self, amplitude: f32) -> Self {
        Self {
            amplitude: amplitude, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, duration_ms, amplitude,
        }
        = self;
        re_export::Input::vibrate_handheld_full(surround_object, duration_ms, amplitude,)
    }
}
#[doc = "Default-param extender for [`Input::action_press_ex`][super::Input::action_press_ex]."]
#[must_use]
pub struct ExActionPress < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Input, action: CowArg < 'a, StringName >, strength: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExActionPress < 'a > {
    fn new(surround_object: &'a mut re_export::Input, action: impl AsArg < StringName > + 'a,) -> Self {
        let strength = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), strength: strength,
        }
    }
    #[inline]
    pub fn strength(self, strength: f32) -> Self {
        Self {
            strength: strength, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, action, strength,
        }
        = self;
        re_export::Input::action_press_full(surround_object, action, strength,)
    }
}
#[doc = "Default-param extender for [`Input::set_default_cursor_shape_ex`][super::Input::set_default_cursor_shape_ex]."]
#[must_use]
pub struct ExSetDefaultCursorShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Input, shape: crate::classes::input::CursorShape,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetDefaultCursorShape < 'a > {
    fn new(surround_object: &'a mut re_export::Input,) -> Self {
        let shape = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shape: shape,
        }
    }
    #[inline]
    pub fn shape(self, shape: crate::classes::input::CursorShape) -> Self {
        Self {
            shape: shape, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shape,
        }
        = self;
        re_export::Input::set_default_cursor_shape_full(surround_object, shape,)
    }
}
#[doc = "Default-param extender for [`Input::set_custom_mouse_cursor_ex`][super::Input::set_custom_mouse_cursor_ex]."]
#[must_use]
pub struct ExSetCustomMouseCursor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Input, image: CowArg < 'a, Option < Gd < crate::classes::Resource > > >, shape: crate::classes::input::CursorShape, hotspot: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCustomMouseCursor < 'a > {
    fn new(surround_object: &'a mut re_export::Input, image: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'a,) -> Self {
        let shape = crate::obj::EngineEnum::from_ord(0);
        let hotspot = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, image: image.into_arg(), shape: shape, hotspot: hotspot,
        }
    }
    #[inline]
    pub fn shape(self, shape: crate::classes::input::CursorShape) -> Self {
        Self {
            shape: shape, .. self
        }
    }
    #[inline]
    pub fn hotspot(self, hotspot: Vector2) -> Self {
        Self {
            hotspot: hotspot, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, image, shape, hotspot,
        }
        = self;
        re_export::Input::set_custom_mouse_cursor_full(surround_object, image, shape, hotspot,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MouseMode {
    ord: i32
}
impl MouseMode {
    #[doc(alias = "MOUSE_MODE_VISIBLE")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_VISIBLE`"]
    pub const VISIBLE: MouseMode = MouseMode {
        ord: 0i32
    };
    #[doc(alias = "MOUSE_MODE_HIDDEN")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_HIDDEN`"]
    pub const HIDDEN: MouseMode = MouseMode {
        ord: 1i32
    };
    #[doc(alias = "MOUSE_MODE_CAPTURED")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CAPTURED`"]
    pub const CAPTURED: MouseMode = MouseMode {
        ord: 2i32
    };
    #[doc(alias = "MOUSE_MODE_CONFINED")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CONFINED`"]
    pub const CONFINED: MouseMode = MouseMode {
        ord: 3i32
    };
    #[doc(alias = "MOUSE_MODE_CONFINED_HIDDEN")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CONFINED_HIDDEN`"]
    pub const CONFINED_HIDDEN: MouseMode = MouseMode {
        ord: 4i32
    };
    #[doc(alias = "MOUSE_MODE_MAX")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_MAX`"]
    pub const MAX: MouseMode = MouseMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for MouseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MouseMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MouseMode {
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
            Self::VISIBLE => "VISIBLE", Self::HIDDEN => "HIDDEN", Self::CAPTURED => "CAPTURED", Self::CONFINED => "CONFINED", Self::CONFINED_HIDDEN => "CONFINED_HIDDEN", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MouseMode::VISIBLE, MouseMode::HIDDEN, MouseMode::CAPTURED, MouseMode::CONFINED, MouseMode::CONFINED_HIDDEN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MouseMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VISIBLE", "MOUSE_MODE_VISIBLE", MouseMode::VISIBLE), crate::meta::inspect::EnumConstant::new("HIDDEN", "MOUSE_MODE_HIDDEN", MouseMode::HIDDEN), crate::meta::inspect::EnumConstant::new("CAPTURED", "MOUSE_MODE_CAPTURED", MouseMode::CAPTURED), crate::meta::inspect::EnumConstant::new("CONFINED", "MOUSE_MODE_CONFINED", MouseMode::CONFINED), crate::meta::inspect::EnumConstant::new("CONFINED_HIDDEN", "MOUSE_MODE_CONFINED_HIDDEN", MouseMode::CONFINED_HIDDEN), crate::meta::inspect::EnumConstant::new("MAX", "MOUSE_MODE_MAX", MouseMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for MouseMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for MouseMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MouseMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MouseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CursorShape {
    ord: i32
}
impl CursorShape {
    #[doc(alias = "CURSOR_ARROW")]
    #[doc = "Godot enumerator name: `CURSOR_ARROW`"]
    pub const ARROW: CursorShape = CursorShape {
        ord: 0i32
    };
    #[doc(alias = "CURSOR_IBEAM")]
    #[doc = "Godot enumerator name: `CURSOR_IBEAM`"]
    pub const IBEAM: CursorShape = CursorShape {
        ord: 1i32
    };
    #[doc(alias = "CURSOR_POINTING_HAND")]
    #[doc = "Godot enumerator name: `CURSOR_POINTING_HAND`"]
    pub const POINTING_HAND: CursorShape = CursorShape {
        ord: 2i32
    };
    #[doc(alias = "CURSOR_CROSS")]
    #[doc = "Godot enumerator name: `CURSOR_CROSS`"]
    pub const CROSS: CursorShape = CursorShape {
        ord: 3i32
    };
    #[doc(alias = "CURSOR_WAIT")]
    #[doc = "Godot enumerator name: `CURSOR_WAIT`"]
    pub const WAIT: CursorShape = CursorShape {
        ord: 4i32
    };
    #[doc(alias = "CURSOR_BUSY")]
    #[doc = "Godot enumerator name: `CURSOR_BUSY`"]
    pub const BUSY: CursorShape = CursorShape {
        ord: 5i32
    };
    #[doc(alias = "CURSOR_DRAG")]
    #[doc = "Godot enumerator name: `CURSOR_DRAG`"]
    pub const DRAG: CursorShape = CursorShape {
        ord: 6i32
    };
    #[doc(alias = "CURSOR_CAN_DROP")]
    #[doc = "Godot enumerator name: `CURSOR_CAN_DROP`"]
    pub const CAN_DROP: CursorShape = CursorShape {
        ord: 7i32
    };
    #[doc(alias = "CURSOR_FORBIDDEN")]
    #[doc = "Godot enumerator name: `CURSOR_FORBIDDEN`"]
    pub const FORBIDDEN: CursorShape = CursorShape {
        ord: 8i32
    };
    #[doc(alias = "CURSOR_VSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_VSIZE`"]
    pub const VSIZE: CursorShape = CursorShape {
        ord: 9i32
    };
    #[doc(alias = "CURSOR_HSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_HSIZE`"]
    pub const HSIZE: CursorShape = CursorShape {
        ord: 10i32
    };
    #[doc(alias = "CURSOR_BDIAGSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_BDIAGSIZE`"]
    pub const BDIAGSIZE: CursorShape = CursorShape {
        ord: 11i32
    };
    #[doc(alias = "CURSOR_FDIAGSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_FDIAGSIZE`"]
    pub const FDIAGSIZE: CursorShape = CursorShape {
        ord: 12i32
    };
    #[doc(alias = "CURSOR_MOVE")]
    #[doc = "Godot enumerator name: `CURSOR_MOVE`"]
    pub const MOVE: CursorShape = CursorShape {
        ord: 13i32
    };
    #[doc(alias = "CURSOR_VSPLIT")]
    #[doc = "Godot enumerator name: `CURSOR_VSPLIT`"]
    pub const VSPLIT: CursorShape = CursorShape {
        ord: 14i32
    };
    #[doc(alias = "CURSOR_HSPLIT")]
    #[doc = "Godot enumerator name: `CURSOR_HSPLIT`"]
    pub const HSPLIT: CursorShape = CursorShape {
        ord: 15i32
    };
    #[doc(alias = "CURSOR_HELP")]
    #[doc = "Godot enumerator name: `CURSOR_HELP`"]
    pub const HELP: CursorShape = CursorShape {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for CursorShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CursorShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CursorShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 => Some(Self {
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
            Self::ARROW => "ARROW", Self::IBEAM => "IBEAM", Self::POINTING_HAND => "POINTING_HAND", Self::CROSS => "CROSS", Self::WAIT => "WAIT", Self::BUSY => "BUSY", Self::DRAG => "DRAG", Self::CAN_DROP => "CAN_DROP", Self::FORBIDDEN => "FORBIDDEN", Self::VSIZE => "VSIZE", Self::HSIZE => "HSIZE", Self::BDIAGSIZE => "BDIAGSIZE", Self::FDIAGSIZE => "FDIAGSIZE", Self::MOVE => "MOVE", Self::VSPLIT => "VSPLIT", Self::HSPLIT => "HSPLIT", Self::HELP => "HELP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CursorShape::ARROW, CursorShape::IBEAM, CursorShape::POINTING_HAND, CursorShape::CROSS, CursorShape::WAIT, CursorShape::BUSY, CursorShape::DRAG, CursorShape::CAN_DROP, CursorShape::FORBIDDEN, CursorShape::VSIZE, CursorShape::HSIZE, CursorShape::BDIAGSIZE, CursorShape::FDIAGSIZE, CursorShape::MOVE, CursorShape::VSPLIT, CursorShape::HSPLIT, CursorShape::HELP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CursorShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ARROW", "CURSOR_ARROW", CursorShape::ARROW), crate::meta::inspect::EnumConstant::new("IBEAM", "CURSOR_IBEAM", CursorShape::IBEAM), crate::meta::inspect::EnumConstant::new("POINTING_HAND", "CURSOR_POINTING_HAND", CursorShape::POINTING_HAND), crate::meta::inspect::EnumConstant::new("CROSS", "CURSOR_CROSS", CursorShape::CROSS), crate::meta::inspect::EnumConstant::new("WAIT", "CURSOR_WAIT", CursorShape::WAIT), crate::meta::inspect::EnumConstant::new("BUSY", "CURSOR_BUSY", CursorShape::BUSY), crate::meta::inspect::EnumConstant::new("DRAG", "CURSOR_DRAG", CursorShape::DRAG), crate::meta::inspect::EnumConstant::new("CAN_DROP", "CURSOR_CAN_DROP", CursorShape::CAN_DROP), crate::meta::inspect::EnumConstant::new("FORBIDDEN", "CURSOR_FORBIDDEN", CursorShape::FORBIDDEN), crate::meta::inspect::EnumConstant::new("VSIZE", "CURSOR_VSIZE", CursorShape::VSIZE), crate::meta::inspect::EnumConstant::new("HSIZE", "CURSOR_HSIZE", CursorShape::HSIZE), crate::meta::inspect::EnumConstant::new("BDIAGSIZE", "CURSOR_BDIAGSIZE", CursorShape::BDIAGSIZE), crate::meta::inspect::EnumConstant::new("FDIAGSIZE", "CURSOR_FDIAGSIZE", CursorShape::FDIAGSIZE), crate::meta::inspect::EnumConstant::new("MOVE", "CURSOR_MOVE", CursorShape::MOVE), crate::meta::inspect::EnumConstant::new("VSPLIT", "CURSOR_VSPLIT", CursorShape::VSPLIT), crate::meta::inspect::EnumConstant::new("HSPLIT", "CURSOR_HSPLIT", CursorShape::HSPLIT), crate::meta::inspect::EnumConstant::new("HELP", "CURSOR_HELP", CursorShape::HELP)]
        }
    }
}
impl crate::meta::GodotConvert for CursorShape {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CursorShape {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CursorShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Input;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Input`][crate::classes::Input] class."]
    pub struct SignalsOfInput < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfInput < 'c, C > {
        #[doc = "Signature: `(device: i64, connected: bool)`"]
        pub fn joy_connection_changed(&mut self) -> SigJoyConnectionChanged < 'c, C > {
            SigJoyConnectionChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "joy_connection_changed")
            }
        }
    }
    type TypedSigJoyConnectionChanged < 'c, C > = TypedSignal < 'c, C, (i64, bool,) >;
    pub struct SigJoyConnectionChanged < 'c, C: WithSignals > {
        typed: TypedSigJoyConnectionChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigJoyConnectionChanged < 'c, C > {
        pub fn emit(&mut self, device: i64, connected: bool,) {
            self.typed.emit_tuple((device, connected,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigJoyConnectionChanged < 'c, C > {
        type Target = TypedSigJoyConnectionChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigJoyConnectionChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Input {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfInput < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfInput < 'c, C > {
        type Target = < < Input as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Input;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfInput < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Input;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}