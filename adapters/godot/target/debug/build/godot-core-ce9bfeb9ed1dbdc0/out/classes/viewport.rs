#![doc = "Sidecar module for class [`Viewport`][crate::classes::Viewport].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Viewport` enums](https://docs.godotengine.org/en/stable/classes/class_viewport.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Viewport.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`viewport`][crate::classes::viewport]: sidecar module with related enum/flag types\n* [`SignalsOfViewport`][crate::classes::viewport::SignalsOfViewport]: signal collection\n\n\nSee also [Godot docs for `Viewport`](https://docs.godotengine.org/en/stable/classes/class_viewport.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Viewport>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Viewport {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Viewport {
        pub fn set_world_2d(&mut self, world_2d: impl AsArg < Option < Gd < crate::classes::World2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::World2D > > >,);
            let args = (world_2d.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10436usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_world_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_world_2d(&self,) -> Option < Gd < crate::classes::World2D > > {
            type CallRet = Option < Gd < crate::classes::World2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10437usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_world_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn find_world_2d(&self,) -> Option < Gd < crate::classes::World2D > > {
            type CallRet = Option < Gd < crate::classes::World2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10438usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "find_world_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_canvas_transform(&mut self, xform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Transform2D,);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10439usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_canvas_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_canvas_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10440usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_canvas_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_global_canvas_transform(&mut self, xform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Transform2D,);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10441usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_global_canvas_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_canvas_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_global_canvas_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stretch_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_stretch_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_final_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_final_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_screen_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_screen_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_rect(&self,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_visible_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transparent_background(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_transparent_background", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_transparent_background(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "has_transparent_background", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_hdr_2d(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_use_hdr_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_hdr_2d(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_using_hdr_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_msaa_2d(&mut self, msaa: crate::classes::viewport::Msaa,) {
            type CallRet = ();
            type CallParams = (crate::classes::viewport::Msaa,);
            let args = (msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10451usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_msaa_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_msaa_2d(&self,) -> crate::classes::viewport::Msaa {
            type CallRet = crate::classes::viewport::Msaa;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10452usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_msaa_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_msaa_3d(&mut self, msaa: crate::classes::viewport::Msaa,) {
            type CallRet = ();
            type CallParams = (crate::classes::viewport::Msaa,);
            let args = (msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10453usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_msaa_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_msaa_3d(&self,) -> crate::classes::viewport::Msaa {
            type CallRet = crate::classes::viewport::Msaa;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_msaa_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_screen_space_aa(&mut self, screen_space_aa: crate::classes::viewport::ScreenSpaceAa,) {
            type CallRet = ();
            type CallParams = (crate::classes::viewport::ScreenSpaceAa,);
            let args = (screen_space_aa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_screen_space_aa", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_screen_space_aa(&self,) -> crate::classes::viewport::ScreenSpaceAa {
            type CallRet = crate::classes::viewport::ScreenSpaceAa;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_screen_space_aa", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_taa(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_use_taa", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_taa(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_using_taa", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_debanding(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_use_debanding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_debanding(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_using_debanding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_occlusion_culling(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_use_occlusion_culling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_occlusion_culling(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_using_occlusion_culling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_debug_draw(&mut self, debug_draw: crate::classes::viewport::DebugDraw,) {
            type CallRet = ();
            type CallParams = (crate::classes::viewport::DebugDraw,);
            let args = (debug_draw,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_debug_draw", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_debug_draw(&self,) -> crate::classes::viewport::DebugDraw {
            type CallRet = crate::classes::viewport::DebugDraw;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_debug_draw", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_oversampling(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_use_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_oversampling(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_using_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_oversampling_override(&mut self, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_oversampling_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_oversampling_override(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_oversampling_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_oversampling(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_render_info(&mut self, type_: crate::classes::viewport::RenderInfoType, info: crate::classes::viewport::RenderInfo,) -> i32 {
            type CallRet = i32;
            type CallParams = (crate::classes::viewport::RenderInfoType, crate::classes::viewport::RenderInfo,);
            let args = (type_, info,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_render_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::ViewportTexture > > {
            type CallRet = Option < Gd < crate::classes::ViewportTexture > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_object_picking(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10472usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_physics_object_picking", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_object_picking(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10473usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_physics_object_picking", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_object_picking_sort(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10474usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_physics_object_picking_sort", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_object_picking_sort(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10475usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_physics_object_picking_sort", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_object_picking_first_only(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_physics_object_picking_first_only", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_object_picking_first_only(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10477usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_physics_object_picking_first_only", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_viewport_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10478usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_viewport_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_text_input(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10479usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "push_text_input", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn push_input_full(&mut self, event: CowArg < Option < Gd < crate::classes::InputEvent > > >, in_local_coords: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::InputEvent > > >, bool,);
            let args = (event, in_local_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10480usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "push_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_input_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_input(&mut self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) {
            self.push_input_ex(event,) . done()
        }
        #[inline]
        pub fn push_input_ex < 'a > (&'a mut self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'a,) -> ExPushInput < 'a > {
            ExPushInput::new(self, event,)
        }
        pub(crate) fn push_unhandled_input_full(&mut self, event: CowArg < Option < Gd < crate::classes::InputEvent > > >, in_local_coords: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::InputEvent > > >, bool,);
            let args = (event, in_local_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10481usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "push_unhandled_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_unhandled_input_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_unhandled_input(&mut self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) {
            self.push_unhandled_input_ex(event,) . done()
        }
        #[inline]
        pub fn push_unhandled_input_ex < 'a > (&'a mut self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'a,) -> ExPushUnhandledInput < 'a > {
            ExPushUnhandledInput::new(self, event,)
        }
        pub fn notify_mouse_entered(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10482usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "notify_mouse_entered", Some(self.__validated_obj()), args,)
            }
        }
        pub fn notify_mouse_exited(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10483usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "notify_mouse_exited", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mouse_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10484usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_mouse_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn warp_mouse(&mut self, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10485usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "warp_mouse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn update_mouse_cursor_state(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10486usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "update_mouse_cursor_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn gui_cancel_drag(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10487usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "gui_cancel_drag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn gui_get_drag_data(&self,) -> Variant {
            type CallRet = Variant;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10488usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "gui_get_drag_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn gui_get_drag_description(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10489usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "gui_get_drag_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn gui_set_drag_description(&mut self, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10490usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "gui_set_drag_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn gui_is_dragging(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10491usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "gui_is_dragging", Some(self.__validated_obj()), args,)
            }
        }
        pub fn gui_is_drag_successful(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10492usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "gui_is_drag_successful", Some(self.__validated_obj()), args,)
            }
        }
        pub fn gui_release_focus(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10493usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "gui_release_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn gui_get_focus_owner(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallRet = Option < Gd < crate::classes::Control > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10494usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "gui_get_focus_owner", Some(self.__validated_obj()), args,)
            }
        }
        pub fn gui_get_hovered_control(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallRet = Option < Gd < crate::classes::Control > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10495usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "gui_get_hovered_control", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disable_input(&mut self, disable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10496usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_disable_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_input_disabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10497usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_input_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_positional_shadow_atlas_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10498usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_positional_shadow_atlas_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_positional_shadow_atlas_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10499usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_positional_shadow_atlas_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_positional_shadow_atlas_16_bits(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10500usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_positional_shadow_atlas_16_bits", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_positional_shadow_atlas_16_bits(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10501usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_positional_shadow_atlas_16_bits", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_snap_controls_to_pixels(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10502usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_snap_controls_to_pixels", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_snap_controls_to_pixels_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10503usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_snap_controls_to_pixels_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_snap_2d_transforms_to_pixel(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10504usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_snap_2d_transforms_to_pixel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_snap_2d_transforms_to_pixel_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10505usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_snap_2d_transforms_to_pixel_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_snap_2d_vertices_to_pixel(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10506usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_snap_2d_vertices_to_pixel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_snap_2d_vertices_to_pixel_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10507usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_snap_2d_vertices_to_pixel_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_positional_shadow_atlas_quadrant_subdiv(&mut self, quadrant: i32, subdiv: crate::classes::viewport::PositionalShadowAtlasQuadrantSubdiv,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::viewport::PositionalShadowAtlasQuadrantSubdiv,);
            let args = (quadrant, subdiv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10508usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_positional_shadow_atlas_quadrant_subdiv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_positional_shadow_atlas_quadrant_subdiv(&self, quadrant: i32,) -> crate::classes::viewport::PositionalShadowAtlasQuadrantSubdiv {
            type CallRet = crate::classes::viewport::PositionalShadowAtlasQuadrantSubdiv;
            type CallParams = (i32,);
            let args = (quadrant,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10509usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_positional_shadow_atlas_quadrant_subdiv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_input_as_handled(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10510usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_input_as_handled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_input_handled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10511usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_input_handled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_handle_input_locally(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10512usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_handle_input_locally", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_handling_input_locally(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10513usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_handling_input_locally", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_default_canvas_item_texture_filter(&mut self, mode: crate::classes::viewport::DefaultCanvasItemTextureFilter,) {
            type CallRet = ();
            type CallParams = (crate::classes::viewport::DefaultCanvasItemTextureFilter,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10514usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_default_canvas_item_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_default_canvas_item_texture_filter(&self,) -> crate::classes::viewport::DefaultCanvasItemTextureFilter {
            type CallRet = crate::classes::viewport::DefaultCanvasItemTextureFilter;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10515usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_default_canvas_item_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_embedding_subwindows(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10516usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_embedding_subwindows", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_embedding_subwindows(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10517usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_embedding_subwindows", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_embedded_subwindows(&self,) -> Array < Gd < crate::classes::Window > > {
            type CallRet = Array < Gd < crate::classes::Window > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10518usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_embedded_subwindows", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_canvas_cull_mask(&mut self, mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10519usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_canvas_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_canvas_cull_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10520usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_canvas_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_canvas_cull_mask_bit(&mut self, layer: u32, enable: bool,) {
            type CallRet = ();
            type CallParams = (u32, bool,);
            let args = (layer, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10521usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_canvas_cull_mask_bit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_canvas_cull_mask_bit(&self, layer: u32,) -> bool {
            type CallRet = bool;
            type CallParams = (u32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10522usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_canvas_cull_mask_bit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_default_canvas_item_texture_repeat(&mut self, mode: crate::classes::viewport::DefaultCanvasItemTextureRepeat,) {
            type CallRet = ();
            type CallParams = (crate::classes::viewport::DefaultCanvasItemTextureRepeat,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10523usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_default_canvas_item_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_default_canvas_item_texture_repeat(&self,) -> crate::classes::viewport::DefaultCanvasItemTextureRepeat {
            type CallRet = crate::classes::viewport::DefaultCanvasItemTextureRepeat;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10524usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_default_canvas_item_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdf_oversize(&mut self, oversize: crate::classes::viewport::SdfOversize,) {
            type CallRet = ();
            type CallParams = (crate::classes::viewport::SdfOversize,);
            let args = (oversize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10525usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_sdf_oversize", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sdf_oversize(&self,) -> crate::classes::viewport::SdfOversize {
            type CallRet = crate::classes::viewport::SdfOversize;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10526usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_sdf_oversize", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdf_scale(&mut self, scale: crate::classes::viewport::SdfScale,) {
            type CallRet = ();
            type CallParams = (crate::classes::viewport::SdfScale,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10527usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_sdf_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sdf_scale(&self,) -> crate::classes::viewport::SdfScale {
            type CallRet = crate::classes::viewport::SdfScale;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10528usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_sdf_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mesh_lod_threshold(&mut self, pixels: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (pixels,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10529usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_mesh_lod_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mesh_lod_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10530usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_mesh_lod_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_as_audio_listener_2d(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10531usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_as_audio_listener_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_audio_listener_2d(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10532usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_audio_listener_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_audio_listener_2d(&self,) -> Option < Gd < crate::classes::AudioListener2D > > {
            type CallRet = Option < Gd < crate::classes::AudioListener2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10533usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_audio_listener_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_camera_2d(&self,) -> Option < Gd < crate::classes::Camera2D > > {
            type CallRet = Option < Gd < crate::classes::Camera2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10534usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_camera_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_world_3d(&mut self, world_3d: impl AsArg < Option < Gd < crate::classes::World3D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::World3D > > >,);
            let args = (world_3d.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10535usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_world_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_world_3d(&self,) -> Option < Gd < crate::classes::World3D > > {
            type CallRet = Option < Gd < crate::classes::World3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10536usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_world_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn find_world_3d(&self,) -> Option < Gd < crate::classes::World3D > > {
            type CallRet = Option < Gd < crate::classes::World3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10537usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "find_world_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_own_world_3d(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10538usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_use_own_world_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_own_world_3d(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10539usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_using_own_world_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_audio_listener_3d(&self,) -> Option < Gd < crate::classes::AudioListener3D > > {
            type CallRet = Option < Gd < crate::classes::AudioListener3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10540usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_audio_listener_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_camera_3d(&self,) -> Option < Gd < crate::classes::Camera3D > > {
            type CallRet = Option < Gd < crate::classes::Camera3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10541usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_camera_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_as_audio_listener_3d(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10542usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_as_audio_listener_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_audio_listener_3d(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_audio_listener_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disable_3d(&mut self, disable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10544usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_disable_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_3d_disabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10545usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_3d_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_xr(&mut self, use_: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10546usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_use_xr", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_xr(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10547usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "is_using_xr", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scaling_3d_mode(&mut self, scaling_3d_mode: crate::classes::viewport::Scaling3DMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::viewport::Scaling3DMode,);
            let args = (scaling_3d_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10548usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_scaling_3d_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scaling_3d_mode(&self,) -> crate::classes::viewport::Scaling3DMode {
            type CallRet = crate::classes::viewport::Scaling3DMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10549usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_scaling_3d_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scaling_3d_scale(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10550usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_scaling_3d_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scaling_3d_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10551usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_scaling_3d_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fsr_sharpness(&mut self, fsr_sharpness: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (fsr_sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10552usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_fsr_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fsr_sharpness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10553usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_fsr_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_mipmap_bias(&mut self, texture_mipmap_bias: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (texture_mipmap_bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10554usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_texture_mipmap_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_mipmap_bias(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10555usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_texture_mipmap_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_anisotropic_filtering_level(&mut self, anisotropic_filtering_level: crate::classes::viewport::AnisotropicFiltering,) {
            type CallRet = ();
            type CallParams = (crate::classes::viewport::AnisotropicFiltering,);
            let args = (anisotropic_filtering_level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10556usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_anisotropic_filtering_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_anisotropic_filtering_level(&self,) -> crate::classes::viewport::AnisotropicFiltering {
            type CallRet = crate::classes::viewport::AnisotropicFiltering;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10557usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_anisotropic_filtering_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vrs_mode(&mut self, mode: crate::classes::viewport::VrsMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::viewport::VrsMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10558usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_vrs_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vrs_mode(&self,) -> crate::classes::viewport::VrsMode {
            type CallRet = crate::classes::viewport::VrsMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10559usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_vrs_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vrs_update_mode(&mut self, mode: crate::classes::viewport::VrsUpdateMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::viewport::VrsUpdateMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10560usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_vrs_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vrs_update_mode(&self,) -> crate::classes::viewport::VrsUpdateMode {
            type CallRet = crate::classes::viewport::VrsUpdateMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10561usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_vrs_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vrs_texture(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10562usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "set_vrs_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vrs_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10563usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Viewport", "get_vrs_texture", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Viewport {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Viewport"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Viewport {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Viewport {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Viewport {
        
    }
    impl std::ops::Deref for Viewport {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Viewport {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Viewport__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Viewport` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Viewport::push_input_ex`][super::Viewport::push_input_ex]."]
#[must_use]
pub struct ExPushInput < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Viewport, event: CowArg < 'a, Option < Gd < crate::classes::InputEvent > > >, in_local_coords: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushInput < 'a > {
    fn new(surround_object: &'a mut re_export::Viewport, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'a,) -> Self {
        let in_local_coords = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, event: event.into_arg(), in_local_coords: in_local_coords,
        }
    }
    #[inline]
    pub fn in_local_coords(self, in_local_coords: bool) -> Self {
        Self {
            in_local_coords: in_local_coords, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, event, in_local_coords,
        }
        = self;
        re_export::Viewport::push_input_full(surround_object, event, in_local_coords,)
    }
}
#[doc = "Default-param extender for [`Viewport::push_unhandled_input_ex`][super::Viewport::push_unhandled_input_ex]."]
#[must_use]
pub struct ExPushUnhandledInput < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Viewport, event: CowArg < 'a, Option < Gd < crate::classes::InputEvent > > >, in_local_coords: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushUnhandledInput < 'a > {
    fn new(surround_object: &'a mut re_export::Viewport, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'a,) -> Self {
        let in_local_coords = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, event: event.into_arg(), in_local_coords: in_local_coords,
        }
    }
    #[inline]
    pub fn in_local_coords(self, in_local_coords: bool) -> Self {
        Self {
            in_local_coords: in_local_coords, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, event, in_local_coords,
        }
        = self;
        re_export::Viewport::push_unhandled_input_full(surround_object, event, in_local_coords,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PositionalShadowAtlasQuadrantSubdiv {
    ord: i32
}
impl PositionalShadowAtlasQuadrantSubdiv {
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_DISABLED")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_DISABLED`"]
    pub const DISABLED: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 0i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_1")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_1`"]
    pub const SUBDIV_1: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 1i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_4")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_4`"]
    pub const SUBDIV_4: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 2i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_16")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_16`"]
    pub const SUBDIV_16: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 3i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_64")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_64`"]
    pub const SUBDIV_64: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 4i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_256")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_256`"]
    pub const SUBDIV_256: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 5i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_1024")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_1024`"]
    pub const SUBDIV_1024: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 6i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_MAX")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_MAX`"]
    pub const MAX: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for PositionalShadowAtlasQuadrantSubdiv {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PositionalShadowAtlasQuadrantSubdiv") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PositionalShadowAtlasQuadrantSubdiv {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::SUBDIV_1 => "SUBDIV_1", Self::SUBDIV_4 => "SUBDIV_4", Self::SUBDIV_16 => "SUBDIV_16", Self::SUBDIV_64 => "SUBDIV_64", Self::SUBDIV_256 => "SUBDIV_256", Self::SUBDIV_1024 => "SUBDIV_1024", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PositionalShadowAtlasQuadrantSubdiv::DISABLED, PositionalShadowAtlasQuadrantSubdiv::SUBDIV_1, PositionalShadowAtlasQuadrantSubdiv::SUBDIV_4, PositionalShadowAtlasQuadrantSubdiv::SUBDIV_16, PositionalShadowAtlasQuadrantSubdiv::SUBDIV_64, PositionalShadowAtlasQuadrantSubdiv::SUBDIV_256, PositionalShadowAtlasQuadrantSubdiv::SUBDIV_1024]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PositionalShadowAtlasQuadrantSubdiv >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "SHADOW_ATLAS_QUADRANT_SUBDIV_DISABLED", PositionalShadowAtlasQuadrantSubdiv::DISABLED), crate::meta::inspect::EnumConstant::new("SUBDIV_1", "SHADOW_ATLAS_QUADRANT_SUBDIV_1", PositionalShadowAtlasQuadrantSubdiv::SUBDIV_1), crate::meta::inspect::EnumConstant::new("SUBDIV_4", "SHADOW_ATLAS_QUADRANT_SUBDIV_4", PositionalShadowAtlasQuadrantSubdiv::SUBDIV_4), crate::meta::inspect::EnumConstant::new("SUBDIV_16", "SHADOW_ATLAS_QUADRANT_SUBDIV_16", PositionalShadowAtlasQuadrantSubdiv::SUBDIV_16), crate::meta::inspect::EnumConstant::new("SUBDIV_64", "SHADOW_ATLAS_QUADRANT_SUBDIV_64", PositionalShadowAtlasQuadrantSubdiv::SUBDIV_64), crate::meta::inspect::EnumConstant::new("SUBDIV_256", "SHADOW_ATLAS_QUADRANT_SUBDIV_256", PositionalShadowAtlasQuadrantSubdiv::SUBDIV_256), crate::meta::inspect::EnumConstant::new("SUBDIV_1024", "SHADOW_ATLAS_QUADRANT_SUBDIV_1024", PositionalShadowAtlasQuadrantSubdiv::SUBDIV_1024), crate::meta::inspect::EnumConstant::new("MAX", "SHADOW_ATLAS_QUADRANT_SUBDIV_MAX", PositionalShadowAtlasQuadrantSubdiv::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for PositionalShadowAtlasQuadrantSubdiv {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for PositionalShadowAtlasQuadrantSubdiv {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PositionalShadowAtlasQuadrantSubdiv {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PositionalShadowAtlasQuadrantSubdiv {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Scaling3DMode {
    ord: i32
}
impl Scaling3DMode {
    #[doc(alias = "SCALING_3D_MODE_BILINEAR")]
    #[doc = "Godot enumerator name: `SCALING_3D_MODE_BILINEAR`"]
    pub const BILINEAR: Scaling3DMode = Scaling3DMode {
        ord: 0i32
    };
    #[doc(alias = "SCALING_3D_MODE_FSR")]
    #[doc = "Godot enumerator name: `SCALING_3D_MODE_FSR`"]
    pub const FSR: Scaling3DMode = Scaling3DMode {
        ord: 1i32
    };
    #[doc(alias = "SCALING_3D_MODE_FSR2")]
    #[doc = "Godot enumerator name: `SCALING_3D_MODE_FSR2`"]
    pub const FSR2: Scaling3DMode = Scaling3DMode {
        ord: 2i32
    };
    #[doc(alias = "SCALING_3D_MODE_METALFX_SPATIAL")]
    #[doc = "Godot enumerator name: `SCALING_3D_MODE_METALFX_SPATIAL`"]
    pub const METALFX_SPATIAL: Scaling3DMode = Scaling3DMode {
        ord: 3i32
    };
    #[doc(alias = "SCALING_3D_MODE_METALFX_TEMPORAL")]
    #[doc = "Godot enumerator name: `SCALING_3D_MODE_METALFX_TEMPORAL`"]
    pub const METALFX_TEMPORAL: Scaling3DMode = Scaling3DMode {
        ord: 4i32
    };
    #[doc(alias = "SCALING_3D_MODE_MAX")]
    #[doc = "Godot enumerator name: `SCALING_3D_MODE_MAX`"]
    pub const MAX: Scaling3DMode = Scaling3DMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for Scaling3DMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Scaling3DMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Scaling3DMode {
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
            Self::BILINEAR => "BILINEAR", Self::FSR => "FSR", Self::FSR2 => "FSR2", Self::METALFX_SPATIAL => "METALFX_SPATIAL", Self::METALFX_TEMPORAL => "METALFX_TEMPORAL", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Scaling3DMode::BILINEAR, Scaling3DMode::FSR, Scaling3DMode::FSR2, Scaling3DMode::METALFX_SPATIAL, Scaling3DMode::METALFX_TEMPORAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Scaling3DMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BILINEAR", "SCALING_3D_MODE_BILINEAR", Scaling3DMode::BILINEAR), crate::meta::inspect::EnumConstant::new("FSR", "SCALING_3D_MODE_FSR", Scaling3DMode::FSR), crate::meta::inspect::EnumConstant::new("FSR2", "SCALING_3D_MODE_FSR2", Scaling3DMode::FSR2), crate::meta::inspect::EnumConstant::new("METALFX_SPATIAL", "SCALING_3D_MODE_METALFX_SPATIAL", Scaling3DMode::METALFX_SPATIAL), crate::meta::inspect::EnumConstant::new("METALFX_TEMPORAL", "SCALING_3D_MODE_METALFX_TEMPORAL", Scaling3DMode::METALFX_TEMPORAL), crate::meta::inspect::EnumConstant::new("MAX", "SCALING_3D_MODE_MAX", Scaling3DMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Scaling3DMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for Scaling3DMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Scaling3DMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Scaling3DMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `MSAA`."]
pub struct Msaa {
    ord: i32
}
impl Msaa {
    #[doc(alias = "MSAA_DISABLED")]
    #[doc = "Godot enumerator name: `MSAA_DISABLED`"]
    pub const DISABLED: Msaa = Msaa {
        ord: 0i32
    };
    pub const MSAA_2X: Msaa = Msaa {
        ord: 1i32
    };
    pub const MSAA_4X: Msaa = Msaa {
        ord: 2i32
    };
    pub const MSAA_8X: Msaa = Msaa {
        ord: 3i32
    };
    #[doc(alias = "MSAA_MAX")]
    #[doc = "Godot enumerator name: `MSAA_MAX`"]
    pub const MAX: Msaa = Msaa {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for Msaa {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Msaa") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Msaa {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::MSAA_2X => "MSAA_2X", Self::MSAA_4X => "MSAA_4X", Self::MSAA_8X => "MSAA_8X", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Msaa::DISABLED, Msaa::MSAA_2X, Msaa::MSAA_4X, Msaa::MSAA_8X]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Msaa >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "MSAA_DISABLED", Msaa::DISABLED), crate::meta::inspect::EnumConstant::new("MSAA_2X", "MSAA_2X", Msaa::MSAA_2X), crate::meta::inspect::EnumConstant::new("MSAA_4X", "MSAA_4X", Msaa::MSAA_4X), crate::meta::inspect::EnumConstant::new("MSAA_8X", "MSAA_8X", Msaa::MSAA_8X), crate::meta::inspect::EnumConstant::new("MAX", "MSAA_MAX", Msaa::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Msaa {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for Msaa {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Msaa {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Msaa {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnisotropicFiltering {
    ord: i32
}
impl AnisotropicFiltering {
    #[doc(alias = "ANISOTROPY_DISABLED")]
    #[doc = "Godot enumerator name: `ANISOTROPY_DISABLED`"]
    pub const DISABLED: AnisotropicFiltering = AnisotropicFiltering {
        ord: 0i32
    };
    pub const ANISOTROPY_2X: AnisotropicFiltering = AnisotropicFiltering {
        ord: 1i32
    };
    pub const ANISOTROPY_4X: AnisotropicFiltering = AnisotropicFiltering {
        ord: 2i32
    };
    pub const ANISOTROPY_8X: AnisotropicFiltering = AnisotropicFiltering {
        ord: 3i32
    };
    pub const ANISOTROPY_16X: AnisotropicFiltering = AnisotropicFiltering {
        ord: 4i32
    };
    #[doc(alias = "ANISOTROPY_MAX")]
    #[doc = "Godot enumerator name: `ANISOTROPY_MAX`"]
    pub const MAX: AnisotropicFiltering = AnisotropicFiltering {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for AnisotropicFiltering {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnisotropicFiltering") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnisotropicFiltering {
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
            Self::DISABLED => "DISABLED", Self::ANISOTROPY_2X => "ANISOTROPY_2X", Self::ANISOTROPY_4X => "ANISOTROPY_4X", Self::ANISOTROPY_8X => "ANISOTROPY_8X", Self::ANISOTROPY_16X => "ANISOTROPY_16X", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AnisotropicFiltering::DISABLED, AnisotropicFiltering::ANISOTROPY_2X, AnisotropicFiltering::ANISOTROPY_4X, AnisotropicFiltering::ANISOTROPY_8X, AnisotropicFiltering::ANISOTROPY_16X]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AnisotropicFiltering >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "ANISOTROPY_DISABLED", AnisotropicFiltering::DISABLED), crate::meta::inspect::EnumConstant::new("ANISOTROPY_2X", "ANISOTROPY_2X", AnisotropicFiltering::ANISOTROPY_2X), crate::meta::inspect::EnumConstant::new("ANISOTROPY_4X", "ANISOTROPY_4X", AnisotropicFiltering::ANISOTROPY_4X), crate::meta::inspect::EnumConstant::new("ANISOTROPY_8X", "ANISOTROPY_8X", AnisotropicFiltering::ANISOTROPY_8X), crate::meta::inspect::EnumConstant::new("ANISOTROPY_16X", "ANISOTROPY_16X", AnisotropicFiltering::ANISOTROPY_16X), crate::meta::inspect::EnumConstant::new("MAX", "ANISOTROPY_MAX", AnisotropicFiltering::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for AnisotropicFiltering {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for AnisotropicFiltering {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AnisotropicFiltering {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnisotropicFiltering {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ScreenSpaceAA`."]
pub struct ScreenSpaceAa {
    ord: i32
}
impl ScreenSpaceAa {
    #[doc(alias = "SCREEN_SPACE_AA_DISABLED")]
    #[doc = "Godot enumerator name: `SCREEN_SPACE_AA_DISABLED`"]
    pub const DISABLED: ScreenSpaceAa = ScreenSpaceAa {
        ord: 0i32
    };
    #[doc(alias = "SCREEN_SPACE_AA_FXAA")]
    #[doc = "Godot enumerator name: `SCREEN_SPACE_AA_FXAA`"]
    pub const FXAA: ScreenSpaceAa = ScreenSpaceAa {
        ord: 1i32
    };
    #[doc(alias = "SCREEN_SPACE_AA_SMAA")]
    #[doc = "Godot enumerator name: `SCREEN_SPACE_AA_SMAA`"]
    pub const SMAA: ScreenSpaceAa = ScreenSpaceAa {
        ord: 2i32
    };
    #[doc(alias = "SCREEN_SPACE_AA_MAX")]
    #[doc = "Godot enumerator name: `SCREEN_SPACE_AA_MAX`"]
    pub const MAX: ScreenSpaceAa = ScreenSpaceAa {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ScreenSpaceAa {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ScreenSpaceAa") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ScreenSpaceAa {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::FXAA => "FXAA", Self::SMAA => "SMAA", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ScreenSpaceAa::DISABLED, ScreenSpaceAa::FXAA, ScreenSpaceAa::SMAA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ScreenSpaceAa >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "SCREEN_SPACE_AA_DISABLED", ScreenSpaceAa::DISABLED), crate::meta::inspect::EnumConstant::new("FXAA", "SCREEN_SPACE_AA_FXAA", ScreenSpaceAa::FXAA), crate::meta::inspect::EnumConstant::new("SMAA", "SCREEN_SPACE_AA_SMAA", ScreenSpaceAa::SMAA), crate::meta::inspect::EnumConstant::new("MAX", "SCREEN_SPACE_AA_MAX", ScreenSpaceAa::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ScreenSpaceAa {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ScreenSpaceAa {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ScreenSpaceAa {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ScreenSpaceAa {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderInfo {
    ord: i32
}
impl RenderInfo {
    #[doc(alias = "RENDER_INFO_OBJECTS_IN_FRAME")]
    #[doc = "Godot enumerator name: `RENDER_INFO_OBJECTS_IN_FRAME`"]
    pub const OBJECTS_IN_FRAME: RenderInfo = RenderInfo {
        ord: 0i32
    };
    #[doc(alias = "RENDER_INFO_PRIMITIVES_IN_FRAME")]
    #[doc = "Godot enumerator name: `RENDER_INFO_PRIMITIVES_IN_FRAME`"]
    pub const PRIMITIVES_IN_FRAME: RenderInfo = RenderInfo {
        ord: 1i32
    };
    #[doc(alias = "RENDER_INFO_DRAW_CALLS_IN_FRAME")]
    #[doc = "Godot enumerator name: `RENDER_INFO_DRAW_CALLS_IN_FRAME`"]
    pub const DRAW_CALLS_IN_FRAME: RenderInfo = RenderInfo {
        ord: 2i32
    };
    #[doc(alias = "RENDER_INFO_MAX")]
    #[doc = "Godot enumerator name: `RENDER_INFO_MAX`"]
    pub const MAX: RenderInfo = RenderInfo {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for RenderInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RenderInfo") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RenderInfo {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::OBJECTS_IN_FRAME => "OBJECTS_IN_FRAME", Self::PRIMITIVES_IN_FRAME => "PRIMITIVES_IN_FRAME", Self::DRAW_CALLS_IN_FRAME => "DRAW_CALLS_IN_FRAME", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[RenderInfo::OBJECTS_IN_FRAME, RenderInfo::PRIMITIVES_IN_FRAME, RenderInfo::DRAW_CALLS_IN_FRAME]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < RenderInfo >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OBJECTS_IN_FRAME", "RENDER_INFO_OBJECTS_IN_FRAME", RenderInfo::OBJECTS_IN_FRAME), crate::meta::inspect::EnumConstant::new("PRIMITIVES_IN_FRAME", "RENDER_INFO_PRIMITIVES_IN_FRAME", RenderInfo::PRIMITIVES_IN_FRAME), crate::meta::inspect::EnumConstant::new("DRAW_CALLS_IN_FRAME", "RENDER_INFO_DRAW_CALLS_IN_FRAME", RenderInfo::DRAW_CALLS_IN_FRAME), crate::meta::inspect::EnumConstant::new("MAX", "RENDER_INFO_MAX", RenderInfo::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for RenderInfo {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for RenderInfo {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RenderInfo {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RenderInfo {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderInfoType {
    ord: i32
}
impl RenderInfoType {
    #[doc(alias = "RENDER_INFO_TYPE_VISIBLE")]
    #[doc = "Godot enumerator name: `RENDER_INFO_TYPE_VISIBLE`"]
    pub const VISIBLE: RenderInfoType = RenderInfoType {
        ord: 0i32
    };
    #[doc(alias = "RENDER_INFO_TYPE_SHADOW")]
    #[doc = "Godot enumerator name: `RENDER_INFO_TYPE_SHADOW`"]
    pub const SHADOW: RenderInfoType = RenderInfoType {
        ord: 1i32
    };
    #[doc(alias = "RENDER_INFO_TYPE_CANVAS")]
    #[doc = "Godot enumerator name: `RENDER_INFO_TYPE_CANVAS`"]
    pub const CANVAS: RenderInfoType = RenderInfoType {
        ord: 2i32
    };
    #[doc(alias = "RENDER_INFO_TYPE_MAX")]
    #[doc = "Godot enumerator name: `RENDER_INFO_TYPE_MAX`"]
    pub const MAX: RenderInfoType = RenderInfoType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for RenderInfoType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RenderInfoType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RenderInfoType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::VISIBLE => "VISIBLE", Self::SHADOW => "SHADOW", Self::CANVAS => "CANVAS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[RenderInfoType::VISIBLE, RenderInfoType::SHADOW, RenderInfoType::CANVAS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < RenderInfoType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VISIBLE", "RENDER_INFO_TYPE_VISIBLE", RenderInfoType::VISIBLE), crate::meta::inspect::EnumConstant::new("SHADOW", "RENDER_INFO_TYPE_SHADOW", RenderInfoType::SHADOW), crate::meta::inspect::EnumConstant::new("CANVAS", "RENDER_INFO_TYPE_CANVAS", RenderInfoType::CANVAS), crate::meta::inspect::EnumConstant::new("MAX", "RENDER_INFO_TYPE_MAX", RenderInfoType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for RenderInfoType {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for RenderInfoType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RenderInfoType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RenderInfoType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DebugDraw {
    ord: i32
}
impl DebugDraw {
    #[doc(alias = "DEBUG_DRAW_DISABLED")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_DISABLED`"]
    pub const DISABLED: DebugDraw = DebugDraw {
        ord: 0i32
    };
    #[doc(alias = "DEBUG_DRAW_UNSHADED")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_UNSHADED`"]
    pub const UNSHADED: DebugDraw = DebugDraw {
        ord: 1i32
    };
    #[doc(alias = "DEBUG_DRAW_LIGHTING")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_LIGHTING`"]
    pub const LIGHTING: DebugDraw = DebugDraw {
        ord: 2i32
    };
    #[doc(alias = "DEBUG_DRAW_OVERDRAW")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_OVERDRAW`"]
    pub const OVERDRAW: DebugDraw = DebugDraw {
        ord: 3i32
    };
    #[doc(alias = "DEBUG_DRAW_WIREFRAME")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_WIREFRAME`"]
    pub const WIREFRAME: DebugDraw = DebugDraw {
        ord: 4i32
    };
    #[doc(alias = "DEBUG_DRAW_NORMAL_BUFFER")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_NORMAL_BUFFER`"]
    pub const NORMAL_BUFFER: DebugDraw = DebugDraw {
        ord: 5i32
    };
    #[doc(alias = "DEBUG_DRAW_VOXEL_GI_ALBEDO")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_VOXEL_GI_ALBEDO`"]
    pub const VOXEL_GI_ALBEDO: DebugDraw = DebugDraw {
        ord: 6i32
    };
    #[doc(alias = "DEBUG_DRAW_VOXEL_GI_LIGHTING")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_VOXEL_GI_LIGHTING`"]
    pub const VOXEL_GI_LIGHTING: DebugDraw = DebugDraw {
        ord: 7i32
    };
    #[doc(alias = "DEBUG_DRAW_VOXEL_GI_EMISSION")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_VOXEL_GI_EMISSION`"]
    pub const VOXEL_GI_EMISSION: DebugDraw = DebugDraw {
        ord: 8i32
    };
    #[doc(alias = "DEBUG_DRAW_SHADOW_ATLAS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_SHADOW_ATLAS`"]
    pub const SHADOW_ATLAS: DebugDraw = DebugDraw {
        ord: 9i32
    };
    #[doc(alias = "DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS`"]
    pub const DIRECTIONAL_SHADOW_ATLAS: DebugDraw = DebugDraw {
        ord: 10i32
    };
    #[doc(alias = "DEBUG_DRAW_SCENE_LUMINANCE")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_SCENE_LUMINANCE`"]
    pub const SCENE_LUMINANCE: DebugDraw = DebugDraw {
        ord: 11i32
    };
    #[doc(alias = "DEBUG_DRAW_SSAO")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_SSAO`"]
    pub const SSAO: DebugDraw = DebugDraw {
        ord: 12i32
    };
    #[doc(alias = "DEBUG_DRAW_SSIL")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_SSIL`"]
    pub const SSIL: DebugDraw = DebugDraw {
        ord: 13i32
    };
    #[doc(alias = "DEBUG_DRAW_PSSM_SPLITS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_PSSM_SPLITS`"]
    pub const PSSM_SPLITS: DebugDraw = DebugDraw {
        ord: 14i32
    };
    #[doc(alias = "DEBUG_DRAW_DECAL_ATLAS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_DECAL_ATLAS`"]
    pub const DECAL_ATLAS: DebugDraw = DebugDraw {
        ord: 15i32
    };
    #[doc(alias = "DEBUG_DRAW_SDFGI")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_SDFGI`"]
    pub const SDFGI: DebugDraw = DebugDraw {
        ord: 16i32
    };
    #[doc(alias = "DEBUG_DRAW_SDFGI_PROBES")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_SDFGI_PROBES`"]
    pub const SDFGI_PROBES: DebugDraw = DebugDraw {
        ord: 17i32
    };
    #[doc(alias = "DEBUG_DRAW_GI_BUFFER")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_GI_BUFFER`"]
    pub const GI_BUFFER: DebugDraw = DebugDraw {
        ord: 18i32
    };
    #[doc(alias = "DEBUG_DRAW_DISABLE_LOD")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_DISABLE_LOD`"]
    pub const DISABLE_LOD: DebugDraw = DebugDraw {
        ord: 19i32
    };
    #[doc(alias = "DEBUG_DRAW_CLUSTER_OMNI_LIGHTS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_CLUSTER_OMNI_LIGHTS`"]
    pub const CLUSTER_OMNI_LIGHTS: DebugDraw = DebugDraw {
        ord: 20i32
    };
    #[doc(alias = "DEBUG_DRAW_CLUSTER_SPOT_LIGHTS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_CLUSTER_SPOT_LIGHTS`"]
    pub const CLUSTER_SPOT_LIGHTS: DebugDraw = DebugDraw {
        ord: 21i32
    };
    #[doc(alias = "DEBUG_DRAW_CLUSTER_DECALS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_CLUSTER_DECALS`"]
    pub const CLUSTER_DECALS: DebugDraw = DebugDraw {
        ord: 22i32
    };
    #[doc(alias = "DEBUG_DRAW_CLUSTER_REFLECTION_PROBES")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_CLUSTER_REFLECTION_PROBES`"]
    pub const CLUSTER_REFLECTION_PROBES: DebugDraw = DebugDraw {
        ord: 23i32
    };
    #[doc(alias = "DEBUG_DRAW_OCCLUDERS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_OCCLUDERS`"]
    pub const OCCLUDERS: DebugDraw = DebugDraw {
        ord: 24i32
    };
    #[doc(alias = "DEBUG_DRAW_MOTION_VECTORS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_MOTION_VECTORS`"]
    pub const MOTION_VECTORS: DebugDraw = DebugDraw {
        ord: 25i32
    };
    #[doc(alias = "DEBUG_DRAW_INTERNAL_BUFFER")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_INTERNAL_BUFFER`"]
    pub const INTERNAL_BUFFER: DebugDraw = DebugDraw {
        ord: 26i32
    };
    
}
impl std::fmt::Debug for DebugDraw {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DebugDraw") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DebugDraw {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::UNSHADED => "UNSHADED", Self::LIGHTING => "LIGHTING", Self::OVERDRAW => "OVERDRAW", Self::WIREFRAME => "WIREFRAME", Self::NORMAL_BUFFER => "NORMAL_BUFFER", Self::VOXEL_GI_ALBEDO => "VOXEL_GI_ALBEDO", Self::VOXEL_GI_LIGHTING => "VOXEL_GI_LIGHTING", Self::VOXEL_GI_EMISSION => "VOXEL_GI_EMISSION", Self::SHADOW_ATLAS => "SHADOW_ATLAS", Self::DIRECTIONAL_SHADOW_ATLAS => "DIRECTIONAL_SHADOW_ATLAS", Self::SCENE_LUMINANCE => "SCENE_LUMINANCE", Self::SSAO => "SSAO", Self::SSIL => "SSIL", Self::PSSM_SPLITS => "PSSM_SPLITS", Self::DECAL_ATLAS => "DECAL_ATLAS", Self::SDFGI => "SDFGI", Self::SDFGI_PROBES => "SDFGI_PROBES", Self::GI_BUFFER => "GI_BUFFER", Self::DISABLE_LOD => "DISABLE_LOD", Self::CLUSTER_OMNI_LIGHTS => "CLUSTER_OMNI_LIGHTS", Self::CLUSTER_SPOT_LIGHTS => "CLUSTER_SPOT_LIGHTS", Self::CLUSTER_DECALS => "CLUSTER_DECALS", Self::CLUSTER_REFLECTION_PROBES => "CLUSTER_REFLECTION_PROBES", Self::OCCLUDERS => "OCCLUDERS", Self::MOTION_VECTORS => "MOTION_VECTORS", Self::INTERNAL_BUFFER => "INTERNAL_BUFFER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DebugDraw::DISABLED, DebugDraw::UNSHADED, DebugDraw::LIGHTING, DebugDraw::OVERDRAW, DebugDraw::WIREFRAME, DebugDraw::NORMAL_BUFFER, DebugDraw::VOXEL_GI_ALBEDO, DebugDraw::VOXEL_GI_LIGHTING, DebugDraw::VOXEL_GI_EMISSION, DebugDraw::SHADOW_ATLAS, DebugDraw::DIRECTIONAL_SHADOW_ATLAS, DebugDraw::SCENE_LUMINANCE, DebugDraw::SSAO, DebugDraw::SSIL, DebugDraw::PSSM_SPLITS, DebugDraw::DECAL_ATLAS, DebugDraw::SDFGI, DebugDraw::SDFGI_PROBES, DebugDraw::GI_BUFFER, DebugDraw::DISABLE_LOD, DebugDraw::CLUSTER_OMNI_LIGHTS, DebugDraw::CLUSTER_SPOT_LIGHTS, DebugDraw::CLUSTER_DECALS, DebugDraw::CLUSTER_REFLECTION_PROBES, DebugDraw::OCCLUDERS, DebugDraw::MOTION_VECTORS, DebugDraw::INTERNAL_BUFFER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DebugDraw >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "DEBUG_DRAW_DISABLED", DebugDraw::DISABLED), crate::meta::inspect::EnumConstant::new("UNSHADED", "DEBUG_DRAW_UNSHADED", DebugDraw::UNSHADED), crate::meta::inspect::EnumConstant::new("LIGHTING", "DEBUG_DRAW_LIGHTING", DebugDraw::LIGHTING), crate::meta::inspect::EnumConstant::new("OVERDRAW", "DEBUG_DRAW_OVERDRAW", DebugDraw::OVERDRAW), crate::meta::inspect::EnumConstant::new("WIREFRAME", "DEBUG_DRAW_WIREFRAME", DebugDraw::WIREFRAME), crate::meta::inspect::EnumConstant::new("NORMAL_BUFFER", "DEBUG_DRAW_NORMAL_BUFFER", DebugDraw::NORMAL_BUFFER), crate::meta::inspect::EnumConstant::new("VOXEL_GI_ALBEDO", "DEBUG_DRAW_VOXEL_GI_ALBEDO", DebugDraw::VOXEL_GI_ALBEDO), crate::meta::inspect::EnumConstant::new("VOXEL_GI_LIGHTING", "DEBUG_DRAW_VOXEL_GI_LIGHTING", DebugDraw::VOXEL_GI_LIGHTING), crate::meta::inspect::EnumConstant::new("VOXEL_GI_EMISSION", "DEBUG_DRAW_VOXEL_GI_EMISSION", DebugDraw::VOXEL_GI_EMISSION), crate::meta::inspect::EnumConstant::new("SHADOW_ATLAS", "DEBUG_DRAW_SHADOW_ATLAS", DebugDraw::SHADOW_ATLAS), crate::meta::inspect::EnumConstant::new("DIRECTIONAL_SHADOW_ATLAS", "DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS", DebugDraw::DIRECTIONAL_SHADOW_ATLAS), crate::meta::inspect::EnumConstant::new("SCENE_LUMINANCE", "DEBUG_DRAW_SCENE_LUMINANCE", DebugDraw::SCENE_LUMINANCE), crate::meta::inspect::EnumConstant::new("SSAO", "DEBUG_DRAW_SSAO", DebugDraw::SSAO), crate::meta::inspect::EnumConstant::new("SSIL", "DEBUG_DRAW_SSIL", DebugDraw::SSIL), crate::meta::inspect::EnumConstant::new("PSSM_SPLITS", "DEBUG_DRAW_PSSM_SPLITS", DebugDraw::PSSM_SPLITS), crate::meta::inspect::EnumConstant::new("DECAL_ATLAS", "DEBUG_DRAW_DECAL_ATLAS", DebugDraw::DECAL_ATLAS), crate::meta::inspect::EnumConstant::new("SDFGI", "DEBUG_DRAW_SDFGI", DebugDraw::SDFGI), crate::meta::inspect::EnumConstant::new("SDFGI_PROBES", "DEBUG_DRAW_SDFGI_PROBES", DebugDraw::SDFGI_PROBES), crate::meta::inspect::EnumConstant::new("GI_BUFFER", "DEBUG_DRAW_GI_BUFFER", DebugDraw::GI_BUFFER), crate::meta::inspect::EnumConstant::new("DISABLE_LOD", "DEBUG_DRAW_DISABLE_LOD", DebugDraw::DISABLE_LOD), crate::meta::inspect::EnumConstant::new("CLUSTER_OMNI_LIGHTS", "DEBUG_DRAW_CLUSTER_OMNI_LIGHTS", DebugDraw::CLUSTER_OMNI_LIGHTS), crate::meta::inspect::EnumConstant::new("CLUSTER_SPOT_LIGHTS", "DEBUG_DRAW_CLUSTER_SPOT_LIGHTS", DebugDraw::CLUSTER_SPOT_LIGHTS), crate::meta::inspect::EnumConstant::new("CLUSTER_DECALS", "DEBUG_DRAW_CLUSTER_DECALS", DebugDraw::CLUSTER_DECALS), crate::meta::inspect::EnumConstant::new("CLUSTER_REFLECTION_PROBES", "DEBUG_DRAW_CLUSTER_REFLECTION_PROBES", DebugDraw::CLUSTER_REFLECTION_PROBES), crate::meta::inspect::EnumConstant::new("OCCLUDERS", "DEBUG_DRAW_OCCLUDERS", DebugDraw::OCCLUDERS), crate::meta::inspect::EnumConstant::new("MOTION_VECTORS", "DEBUG_DRAW_MOTION_VECTORS", DebugDraw::MOTION_VECTORS), crate::meta::inspect::EnumConstant::new("INTERNAL_BUFFER", "DEBUG_DRAW_INTERNAL_BUFFER", DebugDraw::INTERNAL_BUFFER)]
        }
    }
}
impl crate::meta::GodotConvert for DebugDraw {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DebugDraw {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DebugDraw {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DefaultCanvasItemTextureFilter {
    ord: i32
}
impl DefaultCanvasItemTextureFilter {
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST`"]
    pub const NEAREST: DefaultCanvasItemTextureFilter = DefaultCanvasItemTextureFilter {
        ord: 0i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR`"]
    pub const LINEAR: DefaultCanvasItemTextureFilter = DefaultCanvasItemTextureFilter {
        ord: 1i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS`"]
    pub const LINEAR_WITH_MIPMAPS: DefaultCanvasItemTextureFilter = DefaultCanvasItemTextureFilter {
        ord: 2i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS`"]
    pub const NEAREST_WITH_MIPMAPS: DefaultCanvasItemTextureFilter = DefaultCanvasItemTextureFilter {
        ord: 3i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_MAX")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_MAX`"]
    pub const MAX: DefaultCanvasItemTextureFilter = DefaultCanvasItemTextureFilter {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for DefaultCanvasItemTextureFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DefaultCanvasItemTextureFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DefaultCanvasItemTextureFilter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::LINEAR_WITH_MIPMAPS => "LINEAR_WITH_MIPMAPS", Self::NEAREST_WITH_MIPMAPS => "NEAREST_WITH_MIPMAPS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DefaultCanvasItemTextureFilter::NEAREST, DefaultCanvasItemTextureFilter::LINEAR, DefaultCanvasItemTextureFilter::LINEAR_WITH_MIPMAPS, DefaultCanvasItemTextureFilter::NEAREST_WITH_MIPMAPS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DefaultCanvasItemTextureFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAREST", "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST", DefaultCanvasItemTextureFilter::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR", DefaultCanvasItemTextureFilter::LINEAR), crate::meta::inspect::EnumConstant::new("LINEAR_WITH_MIPMAPS", "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS", DefaultCanvasItemTextureFilter::LINEAR_WITH_MIPMAPS), crate::meta::inspect::EnumConstant::new("NEAREST_WITH_MIPMAPS", "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS", DefaultCanvasItemTextureFilter::NEAREST_WITH_MIPMAPS), crate::meta::inspect::EnumConstant::new("MAX", "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_MAX", DefaultCanvasItemTextureFilter::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for DefaultCanvasItemTextureFilter {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for DefaultCanvasItemTextureFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DefaultCanvasItemTextureFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DefaultCanvasItemTextureFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DefaultCanvasItemTextureRepeat {
    ord: i32
}
impl DefaultCanvasItemTextureRepeat {
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_DISABLED")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_DISABLED`"]
    pub const DISABLED: DefaultCanvasItemTextureRepeat = DefaultCanvasItemTextureRepeat {
        ord: 0i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_ENABLED")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_ENABLED`"]
    pub const ENABLED: DefaultCanvasItemTextureRepeat = DefaultCanvasItemTextureRepeat {
        ord: 1i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MIRROR")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MIRROR`"]
    pub const MIRROR: DefaultCanvasItemTextureRepeat = DefaultCanvasItemTextureRepeat {
        ord: 2i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MAX")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MAX`"]
    pub const MAX: DefaultCanvasItemTextureRepeat = DefaultCanvasItemTextureRepeat {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for DefaultCanvasItemTextureRepeat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DefaultCanvasItemTextureRepeat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DefaultCanvasItemTextureRepeat {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::MIRROR => "MIRROR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DefaultCanvasItemTextureRepeat::DISABLED, DefaultCanvasItemTextureRepeat::ENABLED, DefaultCanvasItemTextureRepeat::MIRROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DefaultCanvasItemTextureRepeat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_DISABLED", DefaultCanvasItemTextureRepeat::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_ENABLED", DefaultCanvasItemTextureRepeat::ENABLED), crate::meta::inspect::EnumConstant::new("MIRROR", "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MIRROR", DefaultCanvasItemTextureRepeat::MIRROR), crate::meta::inspect::EnumConstant::new("MAX", "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MAX", DefaultCanvasItemTextureRepeat::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for DefaultCanvasItemTextureRepeat {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for DefaultCanvasItemTextureRepeat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DefaultCanvasItemTextureRepeat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DefaultCanvasItemTextureRepeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `SDFOversize`."]
pub struct SdfOversize {
    ord: i32
}
impl SdfOversize {
    #[doc(alias = "SDF_OVERSIZE_100_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_OVERSIZE_100_PERCENT`"]
    pub const OVERSIZE_100_PERCENT: SdfOversize = SdfOversize {
        ord: 0i32
    };
    #[doc(alias = "SDF_OVERSIZE_120_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_OVERSIZE_120_PERCENT`"]
    pub const OVERSIZE_120_PERCENT: SdfOversize = SdfOversize {
        ord: 1i32
    };
    #[doc(alias = "SDF_OVERSIZE_150_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_OVERSIZE_150_PERCENT`"]
    pub const OVERSIZE_150_PERCENT: SdfOversize = SdfOversize {
        ord: 2i32
    };
    #[doc(alias = "SDF_OVERSIZE_200_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_OVERSIZE_200_PERCENT`"]
    pub const OVERSIZE_200_PERCENT: SdfOversize = SdfOversize {
        ord: 3i32
    };
    #[doc(alias = "SDF_OVERSIZE_MAX")]
    #[doc = "Godot enumerator name: `SDF_OVERSIZE_MAX`"]
    pub const MAX: SdfOversize = SdfOversize {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for SdfOversize {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SdfOversize") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SdfOversize {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::OVERSIZE_100_PERCENT => "OVERSIZE_100_PERCENT", Self::OVERSIZE_120_PERCENT => "OVERSIZE_120_PERCENT", Self::OVERSIZE_150_PERCENT => "OVERSIZE_150_PERCENT", Self::OVERSIZE_200_PERCENT => "OVERSIZE_200_PERCENT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SdfOversize::OVERSIZE_100_PERCENT, SdfOversize::OVERSIZE_120_PERCENT, SdfOversize::OVERSIZE_150_PERCENT, SdfOversize::OVERSIZE_200_PERCENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SdfOversize >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OVERSIZE_100_PERCENT", "SDF_OVERSIZE_100_PERCENT", SdfOversize::OVERSIZE_100_PERCENT), crate::meta::inspect::EnumConstant::new("OVERSIZE_120_PERCENT", "SDF_OVERSIZE_120_PERCENT", SdfOversize::OVERSIZE_120_PERCENT), crate::meta::inspect::EnumConstant::new("OVERSIZE_150_PERCENT", "SDF_OVERSIZE_150_PERCENT", SdfOversize::OVERSIZE_150_PERCENT), crate::meta::inspect::EnumConstant::new("OVERSIZE_200_PERCENT", "SDF_OVERSIZE_200_PERCENT", SdfOversize::OVERSIZE_200_PERCENT), crate::meta::inspect::EnumConstant::new("MAX", "SDF_OVERSIZE_MAX", SdfOversize::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for SdfOversize {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for SdfOversize {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SdfOversize {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SdfOversize {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `SDFScale`."]
pub struct SdfScale {
    ord: i32
}
impl SdfScale {
    #[doc(alias = "SDF_SCALE_100_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_SCALE_100_PERCENT`"]
    pub const SCALE_100_PERCENT: SdfScale = SdfScale {
        ord: 0i32
    };
    #[doc(alias = "SDF_SCALE_50_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_SCALE_50_PERCENT`"]
    pub const SCALE_50_PERCENT: SdfScale = SdfScale {
        ord: 1i32
    };
    #[doc(alias = "SDF_SCALE_25_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_SCALE_25_PERCENT`"]
    pub const SCALE_25_PERCENT: SdfScale = SdfScale {
        ord: 2i32
    };
    #[doc(alias = "SDF_SCALE_MAX")]
    #[doc = "Godot enumerator name: `SDF_SCALE_MAX`"]
    pub const MAX: SdfScale = SdfScale {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for SdfScale {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SdfScale") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SdfScale {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::SCALE_100_PERCENT => "SCALE_100_PERCENT", Self::SCALE_50_PERCENT => "SCALE_50_PERCENT", Self::SCALE_25_PERCENT => "SCALE_25_PERCENT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SdfScale::SCALE_100_PERCENT, SdfScale::SCALE_50_PERCENT, SdfScale::SCALE_25_PERCENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SdfScale >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCALE_100_PERCENT", "SDF_SCALE_100_PERCENT", SdfScale::SCALE_100_PERCENT), crate::meta::inspect::EnumConstant::new("SCALE_50_PERCENT", "SDF_SCALE_50_PERCENT", SdfScale::SCALE_50_PERCENT), crate::meta::inspect::EnumConstant::new("SCALE_25_PERCENT", "SDF_SCALE_25_PERCENT", SdfScale::SCALE_25_PERCENT), crate::meta::inspect::EnumConstant::new("MAX", "SDF_SCALE_MAX", SdfScale::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for SdfScale {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for SdfScale {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SdfScale {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SdfScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `VRSMode`."]
pub struct VrsMode {
    ord: i32
}
impl VrsMode {
    #[doc(alias = "VRS_DISABLED")]
    #[doc = "Godot enumerator name: `VRS_DISABLED`"]
    pub const DISABLED: VrsMode = VrsMode {
        ord: 0i32
    };
    #[doc(alias = "VRS_TEXTURE")]
    #[doc = "Godot enumerator name: `VRS_TEXTURE`"]
    pub const TEXTURE: VrsMode = VrsMode {
        ord: 1i32
    };
    #[doc(alias = "VRS_XR")]
    #[doc = "Godot enumerator name: `VRS_XR`"]
    pub const XR: VrsMode = VrsMode {
        ord: 2i32
    };
    #[doc(alias = "VRS_MAX")]
    #[doc = "Godot enumerator name: `VRS_MAX`"]
    pub const MAX: VrsMode = VrsMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for VrsMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VrsMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VrsMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::TEXTURE => "TEXTURE", Self::XR => "XR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VrsMode::DISABLED, VrsMode::TEXTURE, VrsMode::XR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VrsMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VRS_DISABLED", VrsMode::DISABLED), crate::meta::inspect::EnumConstant::new("TEXTURE", "VRS_TEXTURE", VrsMode::TEXTURE), crate::meta::inspect::EnumConstant::new("XR", "VRS_XR", VrsMode::XR), crate::meta::inspect::EnumConstant::new("MAX", "VRS_MAX", VrsMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for VrsMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for VrsMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VrsMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VrsMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `VRSUpdateMode`."]
pub struct VrsUpdateMode {
    ord: i32
}
impl VrsUpdateMode {
    #[doc(alias = "VRS_UPDATE_DISABLED")]
    #[doc = "Godot enumerator name: `VRS_UPDATE_DISABLED`"]
    pub const DISABLED: VrsUpdateMode = VrsUpdateMode {
        ord: 0i32
    };
    #[doc(alias = "VRS_UPDATE_ONCE")]
    #[doc = "Godot enumerator name: `VRS_UPDATE_ONCE`"]
    pub const ONCE: VrsUpdateMode = VrsUpdateMode {
        ord: 1i32
    };
    #[doc(alias = "VRS_UPDATE_ALWAYS")]
    #[doc = "Godot enumerator name: `VRS_UPDATE_ALWAYS`"]
    pub const ALWAYS: VrsUpdateMode = VrsUpdateMode {
        ord: 2i32
    };
    #[doc(alias = "VRS_UPDATE_MAX")]
    #[doc = "Godot enumerator name: `VRS_UPDATE_MAX`"]
    pub const MAX: VrsUpdateMode = VrsUpdateMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for VrsUpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VrsUpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VrsUpdateMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::ONCE => "ONCE", Self::ALWAYS => "ALWAYS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VrsUpdateMode::DISABLED, VrsUpdateMode::ONCE, VrsUpdateMode::ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VrsUpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VRS_UPDATE_DISABLED", VrsUpdateMode::DISABLED), crate::meta::inspect::EnumConstant::new("ONCE", "VRS_UPDATE_ONCE", VrsUpdateMode::ONCE), crate::meta::inspect::EnumConstant::new("ALWAYS", "VRS_UPDATE_ALWAYS", VrsUpdateMode::ALWAYS), crate::meta::inspect::EnumConstant::new("MAX", "VRS_UPDATE_MAX", VrsUpdateMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for VrsUpdateMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for VrsUpdateMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VrsUpdateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VrsUpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Viewport;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Viewport`][crate::classes::Viewport] class."]
    pub struct SignalsOfViewport < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfViewport < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn size_changed(&mut self) -> SigSizeChanged < 'c, C > {
            SigSizeChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "size_changed")
            }
        }
        #[doc = "Signature: `(node: Gd<Control>)`"]
        pub fn gui_focus_changed(&mut self) -> SigGuiFocusChanged < 'c, C > {
            SigGuiFocusChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "gui_focus_changed")
            }
        }
    }
    type TypedSigSizeChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSizeChanged < 'c, C: WithSignals > {
        typed: TypedSigSizeChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSizeChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSizeChanged < 'c, C > {
        type Target = TypedSigSizeChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSizeChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigGuiFocusChanged < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Control >,) >;
    pub struct SigGuiFocusChanged < 'c, C: WithSignals > {
        typed: TypedSigGuiFocusChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigGuiFocusChanged < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Control >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigGuiFocusChanged < 'c, C > {
        type Target = TypedSigGuiFocusChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigGuiFocusChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Viewport {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfViewport < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfViewport < 'c, C > {
        type Target = < < Viewport as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Viewport;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfViewport < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Viewport;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}