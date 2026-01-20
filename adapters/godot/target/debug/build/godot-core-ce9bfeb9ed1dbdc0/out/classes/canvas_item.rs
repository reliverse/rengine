#![doc = "Sidecar module for class [`CanvasItem`][crate::classes::CanvasItem].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CanvasItem` enums](https://docs.godotengine.org/en/stable/classes/class_canvasitem.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CanvasItem.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`canvas_item`][crate::classes::canvas_item]: sidecar module with related enum/flag types\n* [`SignalsOfCanvasItem`][crate::classes::canvas_item::SignalsOfCanvasItem]: signal collection\n* [`CanvasItemNotification`][crate::classes::notify::CanvasItemNotification]: notification type\n\n\nSee also [Godot docs for `CanvasItem`](https://docs.godotengine.org/en/stable/classes/class_canvasitem.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<CanvasItem>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CanvasItem {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Notification type for class [`CanvasItem`][crate::classes::CanvasItem]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum CanvasItemNotification {
        TRANSFORM_CHANGED = 2000i32, LOCAL_TRANSFORM_CHANGED = 35i32, DRAW = 30i32, VISIBILITY_CHANGED = 31i32, ENTER_CANVAS = 32i32, EXIT_CANVAS = 33i32, WORLD_2D_CHANGED = 36i32, ENTER_TREE = 10i32, EXIT_TREE = 11i32, MOVED_IN_PARENT = 12i32, READY = 13i32, PAUSED = 14i32, UNPAUSED = 15i32, PHYSICS_PROCESS = 16i32, PROCESS = 17i32, PARENTED = 18i32, UNPARENTED = 19i32, SCENE_INSTANTIATED = 20i32, DRAG_BEGIN = 21i32, DRAG_END = 22i32, PATH_RENAMED = 23i32, CHILD_ORDER_CHANGED = 24i32, INTERNAL_PROCESS = 25i32, INTERNAL_PHYSICS_PROCESS = 26i32, POST_ENTER_TREE = 27i32, DISABLED = 28i32, ENABLED = 29i32, RESET_PHYSICS_INTERPOLATION = 2001i32, EDITOR_PRE_SAVE = 9001i32, EDITOR_POST_SAVE = 9002i32, WM_MOUSE_ENTER = 1002i32, WM_MOUSE_EXIT = 1003i32, WM_WINDOW_FOCUS_IN = 1004i32, WM_WINDOW_FOCUS_OUT = 1005i32, WM_CLOSE_REQUEST = 1006i32, WM_GO_BACK_REQUEST = 1007i32, WM_SIZE_CHANGED = 1008i32, WM_DPI_CHANGE = 1009i32, VP_MOUSE_ENTER = 1010i32, VP_MOUSE_EXIT = 1011i32, WM_POSITION_CHANGED = 1012i32, OS_MEMORY_WARNING = 2009i32, TRANSLATION_CHANGED = 2010i32, WM_ABOUT = 2011i32, CRASH = 2012i32, OS_IME_UPDATE = 2013i32, APPLICATION_RESUMED = 2014i32, APPLICATION_PAUSED = 2015i32, APPLICATION_FOCUS_IN = 2016i32, APPLICATION_FOCUS_OUT = 2017i32, TEXT_SERVER_CHANGED = 2018i32, ACCESSIBILITY_UPDATE = 3000i32, ACCESSIBILITY_INVALIDATE = 3001i32, POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for CanvasItemNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                2000i32 => Self::TRANSFORM_CHANGED, 35i32 => Self::LOCAL_TRANSFORM_CHANGED, 30i32 => Self::DRAW, 31i32 => Self::VISIBILITY_CHANGED, 32i32 => Self::ENTER_CANVAS, 33i32 => Self::EXIT_CANVAS, 36i32 => Self::WORLD_2D_CHANGED, 10i32 => Self::ENTER_TREE, 11i32 => Self::EXIT_TREE, 12i32 => Self::MOVED_IN_PARENT, 13i32 => Self::READY, 14i32 => Self::PAUSED, 15i32 => Self::UNPAUSED, 16i32 => Self::PHYSICS_PROCESS, 17i32 => Self::PROCESS, 18i32 => Self::PARENTED, 19i32 => Self::UNPARENTED, 20i32 => Self::SCENE_INSTANTIATED, 21i32 => Self::DRAG_BEGIN, 22i32 => Self::DRAG_END, 23i32 => Self::PATH_RENAMED, 24i32 => Self::CHILD_ORDER_CHANGED, 25i32 => Self::INTERNAL_PROCESS, 26i32 => Self::INTERNAL_PHYSICS_PROCESS, 27i32 => Self::POST_ENTER_TREE, 28i32 => Self::DISABLED, 29i32 => Self::ENABLED, 2001i32 => Self::RESET_PHYSICS_INTERPOLATION, 9001i32 => Self::EDITOR_PRE_SAVE, 9002i32 => Self::EDITOR_POST_SAVE, 1002i32 => Self::WM_MOUSE_ENTER, 1003i32 => Self::WM_MOUSE_EXIT, 1004i32 => Self::WM_WINDOW_FOCUS_IN, 1005i32 => Self::WM_WINDOW_FOCUS_OUT, 1006i32 => Self::WM_CLOSE_REQUEST, 1007i32 => Self::WM_GO_BACK_REQUEST, 1008i32 => Self::WM_SIZE_CHANGED, 1009i32 => Self::WM_DPI_CHANGE, 1010i32 => Self::VP_MOUSE_ENTER, 1011i32 => Self::VP_MOUSE_EXIT, 1012i32 => Self::WM_POSITION_CHANGED, 2009i32 => Self::OS_MEMORY_WARNING, 2010i32 => Self::TRANSLATION_CHANGED, 2011i32 => Self::WM_ABOUT, 2012i32 => Self::CRASH, 2013i32 => Self::OS_IME_UPDATE, 2014i32 => Self::APPLICATION_RESUMED, 2015i32 => Self::APPLICATION_PAUSED, 2016i32 => Self::APPLICATION_FOCUS_IN, 2017i32 => Self::APPLICATION_FOCUS_OUT, 2018i32 => Self::TEXT_SERVER_CHANGED, 3000i32 => Self::ACCESSIBILITY_UPDATE, 3001i32 => Self::ACCESSIBILITY_INVALIDATE, 0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < CanvasItemNotification > for i32 {
        fn from(notification: CanvasItemNotification) -> i32 {
            match notification {
                CanvasItemNotification::TRANSFORM_CHANGED => 2000i32, CanvasItemNotification::LOCAL_TRANSFORM_CHANGED => 35i32, CanvasItemNotification::DRAW => 30i32, CanvasItemNotification::VISIBILITY_CHANGED => 31i32, CanvasItemNotification::ENTER_CANVAS => 32i32, CanvasItemNotification::EXIT_CANVAS => 33i32, CanvasItemNotification::WORLD_2D_CHANGED => 36i32, CanvasItemNotification::ENTER_TREE => 10i32, CanvasItemNotification::EXIT_TREE => 11i32, CanvasItemNotification::MOVED_IN_PARENT => 12i32, CanvasItemNotification::READY => 13i32, CanvasItemNotification::PAUSED => 14i32, CanvasItemNotification::UNPAUSED => 15i32, CanvasItemNotification::PHYSICS_PROCESS => 16i32, CanvasItemNotification::PROCESS => 17i32, CanvasItemNotification::PARENTED => 18i32, CanvasItemNotification::UNPARENTED => 19i32, CanvasItemNotification::SCENE_INSTANTIATED => 20i32, CanvasItemNotification::DRAG_BEGIN => 21i32, CanvasItemNotification::DRAG_END => 22i32, CanvasItemNotification::PATH_RENAMED => 23i32, CanvasItemNotification::CHILD_ORDER_CHANGED => 24i32, CanvasItemNotification::INTERNAL_PROCESS => 25i32, CanvasItemNotification::INTERNAL_PHYSICS_PROCESS => 26i32, CanvasItemNotification::POST_ENTER_TREE => 27i32, CanvasItemNotification::DISABLED => 28i32, CanvasItemNotification::ENABLED => 29i32, CanvasItemNotification::RESET_PHYSICS_INTERPOLATION => 2001i32, CanvasItemNotification::EDITOR_PRE_SAVE => 9001i32, CanvasItemNotification::EDITOR_POST_SAVE => 9002i32, CanvasItemNotification::WM_MOUSE_ENTER => 1002i32, CanvasItemNotification::WM_MOUSE_EXIT => 1003i32, CanvasItemNotification::WM_WINDOW_FOCUS_IN => 1004i32, CanvasItemNotification::WM_WINDOW_FOCUS_OUT => 1005i32, CanvasItemNotification::WM_CLOSE_REQUEST => 1006i32, CanvasItemNotification::WM_GO_BACK_REQUEST => 1007i32, CanvasItemNotification::WM_SIZE_CHANGED => 1008i32, CanvasItemNotification::WM_DPI_CHANGE => 1009i32, CanvasItemNotification::VP_MOUSE_ENTER => 1010i32, CanvasItemNotification::VP_MOUSE_EXIT => 1011i32, CanvasItemNotification::WM_POSITION_CHANGED => 1012i32, CanvasItemNotification::OS_MEMORY_WARNING => 2009i32, CanvasItemNotification::TRANSLATION_CHANGED => 2010i32, CanvasItemNotification::WM_ABOUT => 2011i32, CanvasItemNotification::CRASH => 2012i32, CanvasItemNotification::OS_IME_UPDATE => 2013i32, CanvasItemNotification::APPLICATION_RESUMED => 2014i32, CanvasItemNotification::APPLICATION_PAUSED => 2015i32, CanvasItemNotification::APPLICATION_FOCUS_IN => 2016i32, CanvasItemNotification::APPLICATION_FOCUS_OUT => 2017i32, CanvasItemNotification::TEXT_SERVER_CHANGED => 2018i32, CanvasItemNotification::ACCESSIBILITY_UPDATE => 3000i32, CanvasItemNotification::ACCESSIBILITY_INVALIDATE => 3001i32, CanvasItemNotification::POSTINITIALIZE => 0i32, CanvasItemNotification::PREDELETE => 1i32, CanvasItemNotification::EXTENSION_RELOADED => 2i32, CanvasItemNotification::Unknown(int) => int,
            }
        }
    }
    impl CanvasItem {
        pub fn get_canvas_item(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_canvas_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_visible_in_tree(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_visible_in_tree", Some(self.__validated_obj()), args,)
            }
        }
        pub fn show(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "show", Some(self.__validated_obj()), args,)
            }
        }
        pub fn hide(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "hide", Some(self.__validated_obj()), args,)
            }
        }
        pub fn queue_redraw(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1823usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "queue_redraw", Some(self.__validated_obj()), args,)
            }
        }
        pub fn move_to_front(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1824usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "move_to_front", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_as_top_level(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1825usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_as_top_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_set_as_top_level(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_set_as_top_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_light_mask(&mut self, light_mask: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (light_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_light_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_light_mask(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_light_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_modulate(&mut self, modulate: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_self_modulate(&mut self, self_modulate: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (self_modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_self_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_self_modulate(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_self_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_z_index(&mut self, z_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (z_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_z_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_z_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_z_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_z_as_relative(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_z_as_relative", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_z_relative(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_z_relative", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_y_sort_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_y_sort_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_y_sort_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_y_sort_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_behind_parent(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_draw_behind_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_draw_behind_parent_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_draw_behind_parent_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn draw_line_full(&mut self, from: Vector2, to: Vector2, color: Color, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2, Color, f32, bool,);
            let args = (from, to, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_line(&mut self, from: Vector2, to: Vector2, color: Color,) {
            self.draw_line_ex(from, to, color,) . done()
        }
        #[inline]
        pub fn draw_line_ex < 'a > (&'a mut self, from: Vector2, to: Vector2, color: Color,) -> ExDrawLine < 'a > {
            ExDrawLine::new(self, from, to, color,)
        }
        pub(crate) fn draw_dashed_line_full(&mut self, from: Vector2, to: Vector2, color: Color, width: f32, dash: f32, aligned: bool, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2, Color, f32, f32, bool, bool,);
            let args = (from, to, color, width, dash, aligned, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_dashed_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_dashed_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_dashed_line(&mut self, from: Vector2, to: Vector2, color: Color,) {
            self.draw_dashed_line_ex(from, to, color,) . done()
        }
        #[inline]
        pub fn draw_dashed_line_ex < 'a > (&'a mut self, from: Vector2, to: Vector2, color: Color,) -> ExDrawDashedLine < 'a > {
            ExDrawDashedLine::new(self, from, to, color,)
        }
        pub(crate) fn draw_polyline_full(&mut self, points: RefArg < PackedVector2Array >, color: Color, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >, Color, f32, bool,);
            let args = (points, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_polyline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_polyline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_polyline(&mut self, points: &PackedVector2Array, color: Color,) {
            self.draw_polyline_ex(points, color,) . done()
        }
        #[inline]
        pub fn draw_polyline_ex < 'a > (&'a mut self, points: &'a PackedVector2Array, color: Color,) -> ExDrawPolyline < 'a > {
            ExDrawPolyline::new(self, points, color,)
        }
        pub(crate) fn draw_polyline_colors_full(&mut self, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, f32, bool,);
            let args = (points, colors, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_polyline_colors", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_polyline_colors_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_polyline_colors(&mut self, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.draw_polyline_colors_ex(points, colors,) . done()
        }
        #[inline]
        pub fn draw_polyline_colors_ex < 'a > (&'a mut self, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> ExDrawPolylineColors < 'a > {
            ExDrawPolylineColors::new(self, points, colors,)
        }
        pub(crate) fn draw_arc_full(&mut self, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Vector2, f32, f32, f32, i32, Color, f32, bool,);
            let args = (center, radius, start_angle, end_angle, point_count, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_arc", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_arc_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_arc(&mut self, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color,) {
            self.draw_arc_ex(center, radius, start_angle, end_angle, point_count, color,) . done()
        }
        #[inline]
        pub fn draw_arc_ex < 'a > (&'a mut self, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color,) -> ExDrawArc < 'a > {
            ExDrawArc::new(self, center, radius, start_angle, end_angle, point_count, color,)
        }
        pub(crate) fn draw_multiline_full(&mut self, points: RefArg < PackedVector2Array >, color: Color, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >, Color, f32, bool,);
            let args = (points, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_multiline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_multiline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_multiline(&mut self, points: &PackedVector2Array, color: Color,) {
            self.draw_multiline_ex(points, color,) . done()
        }
        #[inline]
        pub fn draw_multiline_ex < 'a > (&'a mut self, points: &'a PackedVector2Array, color: Color,) -> ExDrawMultiline < 'a > {
            ExDrawMultiline::new(self, points, color,)
        }
        pub(crate) fn draw_multiline_colors_full(&mut self, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, f32, bool,);
            let args = (points, colors, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_multiline_colors", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_multiline_colors_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_multiline_colors(&mut self, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.draw_multiline_colors_ex(points, colors,) . done()
        }
        #[inline]
        pub fn draw_multiline_colors_ex < 'a > (&'a mut self, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> ExDrawMultilineColors < 'a > {
            ExDrawMultilineColors::new(self, points, colors,)
        }
        pub(crate) fn draw_rect_full(&mut self, rect: Rect2, color: Color, filled: bool, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Rect2, Color, bool, f32, bool,);
            let args = (rect, color, filled, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_rect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_rect(&mut self, rect: Rect2, color: Color,) {
            self.draw_rect_ex(rect, color,) . done()
        }
        #[inline]
        pub fn draw_rect_ex < 'a > (&'a mut self, rect: Rect2, color: Color,) -> ExDrawRect < 'a > {
            ExDrawRect::new(self, rect, color,)
        }
        pub(crate) fn draw_circle_full(&mut self, position: Vector2, radius: f32, color: Color, filled: bool, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Vector2, f32, Color, bool, f32, bool,);
            let args = (position, radius, color, filled, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_circle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_circle_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_circle(&mut self, position: Vector2, radius: f32, color: Color,) {
            self.draw_circle_ex(position, radius, color,) . done()
        }
        #[inline]
        pub fn draw_circle_ex < 'a > (&'a mut self, position: Vector2, radius: f32, color: Color,) -> ExDrawCircle < 'a > {
            ExDrawCircle::new(self, position, radius, color,)
        }
        pub(crate) fn draw_texture_full(&mut self, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, position: Vector2, modulate: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, Vector2, Color,);
            let args = (texture, position, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_texture_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_texture(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, position: Vector2,) {
            self.draw_texture_ex(texture, position,) . done()
        }
        #[inline]
        pub fn draw_texture_ex < 'a > (&'a mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, position: Vector2,) -> ExDrawTexture < 'a > {
            ExDrawTexture::new(self, texture, position,)
        }
        pub(crate) fn draw_texture_rect_full(&mut self, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, rect: Rect2, tile: bool, modulate: Color, transpose: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, Rect2, bool, Color, bool,);
            let args = (texture, rect, tile, modulate, transpose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_texture_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_texture_rect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_texture_rect(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, rect: Rect2, tile: bool,) {
            self.draw_texture_rect_ex(texture, rect, tile,) . done()
        }
        #[inline]
        pub fn draw_texture_rect_ex < 'a > (&'a mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, rect: Rect2, tile: bool,) -> ExDrawTextureRect < 'a > {
            ExDrawTextureRect::new(self, texture, rect, tile,)
        }
        pub(crate) fn draw_texture_rect_region_full(&mut self, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, Rect2, Rect2, Color, bool, bool,);
            let args = (texture, rect, src_rect, modulate, transpose, clip_uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_texture_rect_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_texture_rect_region_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_texture_rect_region(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, rect: Rect2, src_rect: Rect2,) {
            self.draw_texture_rect_region_ex(texture, rect, src_rect,) . done()
        }
        #[inline]
        pub fn draw_texture_rect_region_ex < 'a > (&'a mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, rect: Rect2, src_rect: Rect2,) -> ExDrawTextureRectRegion < 'a > {
            ExDrawTextureRectRegion::new(self, texture, rect, src_rect,)
        }
        pub(crate) fn draw_msdf_texture_rect_region_full(&mut self, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, rect: Rect2, src_rect: Rect2, modulate: Color, outline: f64, pixel_range: f64, scale: f64,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, Rect2, Rect2, Color, f64, f64, f64,);
            let args = (texture, rect, src_rect, modulate, outline, pixel_range, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_msdf_texture_rect_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_msdf_texture_rect_region_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_msdf_texture_rect_region(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, rect: Rect2, src_rect: Rect2,) {
            self.draw_msdf_texture_rect_region_ex(texture, rect, src_rect,) . done()
        }
        #[inline]
        pub fn draw_msdf_texture_rect_region_ex < 'a > (&'a mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, rect: Rect2, src_rect: Rect2,) -> ExDrawMsdfTextureRectRegion < 'a > {
            ExDrawMsdfTextureRectRegion::new(self, texture, rect, src_rect,)
        }
        pub(crate) fn draw_lcd_texture_rect_region_full(&mut self, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, rect: Rect2, src_rect: Rect2, modulate: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, Rect2, Rect2, Color,);
            let args = (texture, rect, src_rect, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_lcd_texture_rect_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_lcd_texture_rect_region_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_lcd_texture_rect_region(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, rect: Rect2, src_rect: Rect2,) {
            self.draw_lcd_texture_rect_region_ex(texture, rect, src_rect,) . done()
        }
        #[inline]
        pub fn draw_lcd_texture_rect_region_ex < 'a > (&'a mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, rect: Rect2, src_rect: Rect2,) -> ExDrawLcdTextureRectRegion < 'a > {
            ExDrawLcdTextureRectRegion::new(self, texture, rect, src_rect,)
        }
        pub fn draw_style_box(&mut self, style_box: impl AsArg < Option < Gd < crate::classes::StyleBox >> >, rect: Rect2,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::StyleBox > > >, Rect2,);
            let args = (style_box.into_arg(), rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_style_box", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn draw_primitive_full(&mut self, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, uvs: RefArg < PackedVector2Array >, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, RefArg < 'a2, PackedVector2Array >, CowArg < 'a3, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (points, colors, uvs, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_primitive", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_primitive_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_primitive(&mut self, points: &PackedVector2Array, colors: &PackedColorArray, uvs: &PackedVector2Array,) {
            self.draw_primitive_ex(points, colors, uvs,) . done()
        }
        #[inline]
        pub fn draw_primitive_ex < 'a > (&'a mut self, points: &'a PackedVector2Array, colors: &'a PackedColorArray, uvs: &'a PackedVector2Array,) -> ExDrawPrimitive < 'a > {
            ExDrawPrimitive::new(self, points, colors, uvs,)
        }
        pub(crate) fn draw_polygon_full(&mut self, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, uvs: RefArg < PackedVector2Array >, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, RefArg < 'a2, PackedVector2Array >, CowArg < 'a3, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (points, colors, uvs, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_polygon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_polygon_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_polygon(&mut self, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.draw_polygon_ex(points, colors,) . done()
        }
        #[inline]
        pub fn draw_polygon_ex < 'a > (&'a mut self, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> ExDrawPolygon < 'a > {
            ExDrawPolygon::new(self, points, colors,)
        }
        pub(crate) fn draw_colored_polygon_full(&mut self, points: RefArg < PackedVector2Array >, color: Color, uvs: RefArg < PackedVector2Array >, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, PackedVector2Array >, Color, RefArg < 'a1, PackedVector2Array >, CowArg < 'a2, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (points, color, uvs, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_colored_polygon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_colored_polygon_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_colored_polygon(&mut self, points: &PackedVector2Array, color: Color,) {
            self.draw_colored_polygon_ex(points, color,) . done()
        }
        #[inline]
        pub fn draw_colored_polygon_ex < 'a > (&'a mut self, points: &'a PackedVector2Array, color: Color,) -> ExDrawColoredPolygon < 'a > {
            ExDrawColoredPolygon::new(self, points, color,)
        }
        pub(crate) fn draw_string_full(&self, font: CowArg < Option < Gd < crate::classes::Font > > >, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >, Vector2, CowArg < 'a1, GString >, crate::global::HorizontalAlignment, f32, i32, Color, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation, f32,);
            let args = (font, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_string(&self, font: impl AsArg < Option < Gd < crate::classes::Font >> >, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_string_ex(font, pos, text,) . done()
        }
        #[inline]
        pub fn draw_string_ex < 'a > (&'a self, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, pos: Vector2, text: impl AsArg < GString > + 'a,) -> ExDrawString < 'a > {
            ExDrawString::new(self, font, pos, text,)
        }
        pub(crate) fn draw_multiline_string_full(&self, font: CowArg < Option < Gd < crate::classes::Font > > >, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >, Vector2, CowArg < 'a1, GString >, crate::global::HorizontalAlignment, f32, i32, i32, Color, crate::classes::text_server::LineBreakFlag, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation, f32,);
            let args = (font, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_multiline_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_multiline_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_multiline_string(&self, font: impl AsArg < Option < Gd < crate::classes::Font >> >, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_multiline_string_ex(font, pos, text,) . done()
        }
        #[inline]
        pub fn draw_multiline_string_ex < 'a > (&'a self, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, pos: Vector2, text: impl AsArg < GString > + 'a,) -> ExDrawMultilineString < 'a > {
            ExDrawMultilineString::new(self, font, pos, text,)
        }
        pub(crate) fn draw_string_outline_full(&self, font: CowArg < Option < Gd < crate::classes::Font > > >, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >, Vector2, CowArg < 'a1, GString >, crate::global::HorizontalAlignment, f32, i32, i32, Color, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation, f32,);
            let args = (font, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_string_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_string_outline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_string_outline(&self, font: impl AsArg < Option < Gd < crate::classes::Font >> >, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_string_outline_ex(font, pos, text,) . done()
        }
        #[inline]
        pub fn draw_string_outline_ex < 'a > (&'a self, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, pos: Vector2, text: impl AsArg < GString > + 'a,) -> ExDrawStringOutline < 'a > {
            ExDrawStringOutline::new(self, font, pos, text,)
        }
        pub(crate) fn draw_multiline_string_outline_full(&self, font: CowArg < Option < Gd < crate::classes::Font > > >, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, size: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >, Vector2, CowArg < 'a1, GString >, crate::global::HorizontalAlignment, f32, i32, i32, i32, Color, crate::classes::text_server::LineBreakFlag, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation, f32,);
            let args = (font, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_multiline_string_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_multiline_string_outline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_multiline_string_outline(&self, font: impl AsArg < Option < Gd < crate::classes::Font >> >, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_multiline_string_outline_ex(font, pos, text,) . done()
        }
        #[inline]
        pub fn draw_multiline_string_outline_ex < 'a > (&'a self, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, pos: Vector2, text: impl AsArg < GString > + 'a,) -> ExDrawMultilineStringOutline < 'a > {
            ExDrawMultilineStringOutline::new(self, font, pos, text,)
        }
        pub(crate) fn draw_char_full(&self, font: CowArg < Option < Gd < crate::classes::Font > > >, pos: Vector2, char: CowArg < GString >, font_size: i32, modulate: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >, Vector2, CowArg < 'a1, GString >, i32, Color, f32,);
            let args = (font, pos, char, font_size, modulate, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_char", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_char_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_char(&self, font: impl AsArg < Option < Gd < crate::classes::Font >> >, pos: Vector2, char: impl AsArg < GString >,) {
            self.draw_char_ex(font, pos, char,) . done()
        }
        #[inline]
        pub fn draw_char_ex < 'a > (&'a self, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, pos: Vector2, char: impl AsArg < GString > + 'a,) -> ExDrawChar < 'a > {
            ExDrawChar::new(self, font, pos, char,)
        }
        pub(crate) fn draw_char_outline_full(&self, font: CowArg < Option < Gd < crate::classes::Font > > >, pos: Vector2, char: CowArg < GString >, font_size: i32, size: i32, modulate: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >, Vector2, CowArg < 'a1, GString >, i32, i32, Color, f32,);
            let args = (font, pos, char, font_size, size, modulate, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_char_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_char_outline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_char_outline(&self, font: impl AsArg < Option < Gd < crate::classes::Font >> >, pos: Vector2, char: impl AsArg < GString >,) {
            self.draw_char_outline_ex(font, pos, char,) . done()
        }
        #[inline]
        pub fn draw_char_outline_ex < 'a > (&'a self, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, pos: Vector2, char: impl AsArg < GString > + 'a,) -> ExDrawCharOutline < 'a > {
            ExDrawCharOutline::new(self, font, pos, char,)
        }
        pub(crate) fn draw_mesh_full(&mut self, mesh: CowArg < Option < Gd < crate::classes::Mesh > > >, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, transform: Transform2D, modulate: Color,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Mesh > > >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, Transform2D, Color,);
            let args = (mesh, texture, transform, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_mesh_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_mesh(&mut self, mesh: impl AsArg < Option < Gd < crate::classes::Mesh >> >, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            self.draw_mesh_ex(mesh, texture,) . done()
        }
        #[inline]
        pub fn draw_mesh_ex < 'a > (&'a mut self, mesh: impl AsArg < Option < Gd < crate::classes::Mesh >> > + 'a, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a,) -> ExDrawMesh < 'a > {
            ExDrawMesh::new(self, mesh, texture,)
        }
        pub fn draw_multimesh(&mut self, multimesh: impl AsArg < Option < Gd < crate::classes::MultiMesh >> >, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::MultiMesh > > >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (multimesh.into_arg(), texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_multimesh", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn draw_set_transform_full(&mut self, position: Vector2, rotation: f32, scale: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, f32, Vector2,);
            let args = (position, rotation, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_set_transform_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_set_transform(&mut self, position: Vector2,) {
            self.draw_set_transform_ex(position,) . done()
        }
        #[inline]
        pub fn draw_set_transform_ex < 'a > (&'a mut self, position: Vector2,) -> ExDrawSetTransform < 'a > {
            ExDrawSetTransform::new(self, position,)
        }
        pub fn draw_set_transform_matrix(&mut self, xform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Transform2D,);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_set_transform_matrix", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn draw_animation_slice_full(&mut self, animation_length: f64, slice_begin: f64, slice_end: f64, offset: f64,) {
            type CallRet = ();
            type CallParams = (f64, f64, f64, f64,);
            let args = (animation_length, slice_begin, slice_end, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_animation_slice", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_animation_slice_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_animation_slice(&mut self, animation_length: f64, slice_begin: f64, slice_end: f64,) {
            self.draw_animation_slice_ex(animation_length, slice_begin, slice_end,) . done()
        }
        #[inline]
        pub fn draw_animation_slice_ex < 'a > (&'a mut self, animation_length: f64, slice_begin: f64, slice_end: f64,) -> ExDrawAnimationSlice < 'a > {
            ExDrawAnimationSlice::new(self, animation_length, slice_begin, slice_end,)
        }
        pub fn draw_end_animation(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_end_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_global_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_transform_with_canvas(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_global_transform_with_canvas", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_viewport_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_viewport_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_viewport_rect(&self,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1875usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_viewport_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_canvas_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_canvas_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_screen_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_screen_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_local_mouse_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_local_mouse_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_mouse_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_global_mouse_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_canvas(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_canvas", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_canvas_layer_node(&self,) -> Option < Gd < crate::classes::CanvasLayer > > {
            type CallRet = Option < Gd < crate::classes::CanvasLayer > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_canvas_layer_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_world_2d(&self,) -> Option < Gd < crate::classes::World2D > > {
            type CallRet = Option < Gd < crate::classes::World2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1882usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_world_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_material(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1883usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_material(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_instance_shader_parameter(&mut self, name: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_instance_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_instance_shader_parameter(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_instance_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_parent_material(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_use_parent_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_parent_material(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1888usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_use_parent_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_notify_local_transform(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1889usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_notify_local_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_local_transform_notification_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_local_transform_notification_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_notify_transform(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_notify_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_transform_notification_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_transform_notification_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn force_update_transform(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "force_update_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn make_canvas_position_local(&self, viewport_point: Vector2,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2,);
            let args = (viewport_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1894usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "make_canvas_position_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn make_input_local(&self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) -> Option < Gd < crate::classes::InputEvent > > {
            type CallRet = Option < Gd < crate::classes::InputEvent > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::InputEvent > > >,);
            let args = (event.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1895usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "make_input_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_layer(&mut self, layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1896usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_visibility_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_layer(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1897usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_visibility_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_layer_bit(&mut self, layer: u32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (u32, bool,);
            let args = (layer, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1898usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_visibility_layer_bit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_layer_bit(&self, layer: u32,) -> bool {
            type CallRet = bool;
            type CallParams = (u32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1899usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_visibility_layer_bit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_filter(&mut self, mode: crate::classes::canvas_item::TextureFilter,) {
            type CallRet = ();
            type CallParams = (crate::classes::canvas_item::TextureFilter,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1900usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::classes::canvas_item::TextureFilter {
            type CallRet = crate::classes::canvas_item::TextureFilter;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1901usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_repeat(&mut self, mode: crate::classes::canvas_item::TextureRepeat,) {
            type CallRet = ();
            type CallParams = (crate::classes::canvas_item::TextureRepeat,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1902usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_repeat(&self,) -> crate::classes::canvas_item::TextureRepeat {
            type CallRet = crate::classes::canvas_item::TextureRepeat;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1903usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clip_children_mode(&mut self, mode: crate::classes::canvas_item::ClipChildrenMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::canvas_item::ClipChildrenMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_clip_children_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_clip_children_mode(&self,) -> crate::classes::canvas_item::ClipChildrenMode {
            type CallRet = crate::classes::canvas_item::ClipChildrenMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_clip_children_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: CanvasItemNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: CanvasItemNotification) {
            self.notification(i32::from(what), true);
            
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
        pub(crate) const NOTIFICATION_TRANSFORM_CHANGED: i32 = 2000i32;
        pub(crate) const NOTIFICATION_LOCAL_TRANSFORM_CHANGED: i32 = 35i32;
        pub(crate) const NOTIFICATION_DRAW: i32 = 30i32;
        pub(crate) const NOTIFICATION_VISIBILITY_CHANGED: i32 = 31i32;
        pub(crate) const NOTIFICATION_ENTER_CANVAS: i32 = 32i32;
        pub(crate) const NOTIFICATION_EXIT_CANVAS: i32 = 33i32;
        pub(crate) const NOTIFICATION_WORLD_2D_CHANGED: i32 = 36i32;
        
    }
    impl crate::obj::GodotClass for CanvasItem {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CanvasItem"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CanvasItem {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CanvasItem {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CanvasItem {
        
    }
    impl std::ops::Deref for CanvasItem {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CanvasItem {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CanvasItem__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `CanvasItem` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_line_ex`][super::CanvasItem::draw_line_ex]."]
#[must_use]
pub struct ExDrawLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, from: Vector2, to: Vector2, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawLine < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, from: Vector2, to: Vector2, color: Color,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, to: to, color: color, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from, to, color, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_line_full(surround_object, from, to, color, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_dashed_line_ex`][super::CanvasItem::draw_dashed_line_ex]."]
#[must_use]
pub struct ExDrawDashedLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, from: Vector2, to: Vector2, color: Color, width: f32, dash: f32, aligned: bool, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawDashedLine < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, from: Vector2, to: Vector2, color: Color,) -> Self {
        let width = - 1f32;
        let dash = 2f32;
        let aligned = true;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, to: to, color: color, width: width, dash: dash, aligned: aligned, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn dash(self, dash: f32) -> Self {
        Self {
            dash: dash, .. self
        }
    }
    #[inline]
    pub fn aligned(self, aligned: bool) -> Self {
        Self {
            aligned: aligned, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from, to, color, width, dash, aligned, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_dashed_line_full(surround_object, from, to, color, width, dash, aligned, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_polyline_ex`][super::CanvasItem::draw_polyline_ex]."]
#[must_use]
pub struct ExDrawPolyline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, points: CowArg < 'a, PackedVector2Array >, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawPolyline < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: &'a PackedVector2Array, color: Color,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), color: color, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, color, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_polyline_full(surround_object, points.cow_as_arg(), color, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_polyline_colors_ex`][super::CanvasItem::draw_polyline_colors_ex]."]
#[must_use]
pub struct ExDrawPolylineColors < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, points: CowArg < 'a, PackedVector2Array >, colors: CowArg < 'a, PackedColorArray >, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawPolylineColors < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, colors, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_polyline_colors_full(surround_object, points.cow_as_arg(), colors.cow_as_arg(), width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_arc_ex`][super::CanvasItem::draw_arc_ex]."]
#[must_use]
pub struct ExDrawArc < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawArc < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, center: center, radius: radius, start_angle: start_angle, end_angle: end_angle, point_count: point_count, color: color, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, center, radius, start_angle, end_angle, point_count, color, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_arc_full(surround_object, center, radius, start_angle, end_angle, point_count, color, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_multiline_ex`][super::CanvasItem::draw_multiline_ex]."]
#[must_use]
pub struct ExDrawMultiline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, points: CowArg < 'a, PackedVector2Array >, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMultiline < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: &'a PackedVector2Array, color: Color,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), color: color, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, color, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_multiline_full(surround_object, points.cow_as_arg(), color, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_multiline_colors_ex`][super::CanvasItem::draw_multiline_colors_ex]."]
#[must_use]
pub struct ExDrawMultilineColors < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, points: CowArg < 'a, PackedVector2Array >, colors: CowArg < 'a, PackedColorArray >, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMultilineColors < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, colors, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_multiline_colors_full(surround_object, points.cow_as_arg(), colors.cow_as_arg(), width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_rect_ex`][super::CanvasItem::draw_rect_ex]."]
#[must_use]
pub struct ExDrawRect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, rect: Rect2, color: Color, filled: bool, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawRect < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, rect: Rect2, color: Color,) -> Self {
        let filled = true;
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rect: rect, color: color, filled: filled, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn filled(self, filled: bool) -> Self {
        Self {
            filled: filled, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, rect, color, filled, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_rect_full(surround_object, rect, color, filled, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_circle_ex`][super::CanvasItem::draw_circle_ex]."]
#[must_use]
pub struct ExDrawCircle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, position: Vector2, radius: f32, color: Color, filled: bool, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawCircle < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, position: Vector2, radius: f32, color: Color,) -> Self {
        let filled = true;
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, radius: radius, color: color, filled: filled, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn filled(self, filled: bool) -> Self {
        Self {
            filled: filled, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, radius, color, filled, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_circle_full(surround_object, position, radius, color, filled, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_texture_ex`][super::CanvasItem::draw_texture_ex]."]
#[must_use]
pub struct ExDrawTexture < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, texture: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, position: Vector2, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawTexture < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, position: Vector2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), position: position, modulate: modulate,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, position, modulate,
        }
        = self;
        re_export::CanvasItem::draw_texture_full(surround_object, texture, position, modulate,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_texture_rect_ex`][super::CanvasItem::draw_texture_rect_ex]."]
#[must_use]
pub struct ExDrawTextureRect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, texture: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, rect: Rect2, tile: bool, modulate: Color, transpose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawTextureRect < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, rect: Rect2, tile: bool,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let transpose = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), rect: rect, tile: tile, modulate: modulate, transpose: transpose,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn transpose(self, transpose: bool) -> Self {
        Self {
            transpose: transpose, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, rect, tile, modulate, transpose,
        }
        = self;
        re_export::CanvasItem::draw_texture_rect_full(surround_object, texture, rect, tile, modulate, transpose,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_texture_rect_region_ex`][super::CanvasItem::draw_texture_rect_region_ex]."]
#[must_use]
pub struct ExDrawTextureRectRegion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, texture: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawTextureRectRegion < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, rect: Rect2, src_rect: Rect2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let transpose = false;
        let clip_uv = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), rect: rect, src_rect: src_rect, modulate: modulate, transpose: transpose, clip_uv: clip_uv,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn transpose(self, transpose: bool) -> Self {
        Self {
            transpose: transpose, .. self
        }
    }
    #[inline]
    pub fn clip_uv(self, clip_uv: bool) -> Self {
        Self {
            clip_uv: clip_uv, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, rect, src_rect, modulate, transpose, clip_uv,
        }
        = self;
        re_export::CanvasItem::draw_texture_rect_region_full(surround_object, texture, rect, src_rect, modulate, transpose, clip_uv,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_msdf_texture_rect_region_ex`][super::CanvasItem::draw_msdf_texture_rect_region_ex]."]
#[must_use]
pub struct ExDrawMsdfTextureRectRegion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, texture: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, rect: Rect2, src_rect: Rect2, modulate: Color, outline: f64, pixel_range: f64, scale: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMsdfTextureRectRegion < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, rect: Rect2, src_rect: Rect2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let outline = 0f64;
        let pixel_range = 4f64;
        let scale = 1f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), rect: rect, src_rect: src_rect, modulate: modulate, outline: outline, pixel_range: pixel_range, scale: scale,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn outline(self, outline: f64) -> Self {
        Self {
            outline: outline, .. self
        }
    }
    #[inline]
    pub fn pixel_range(self, pixel_range: f64) -> Self {
        Self {
            pixel_range: pixel_range, .. self
        }
    }
    #[inline]
    pub fn scale(self, scale: f64) -> Self {
        Self {
            scale: scale, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, rect, src_rect, modulate, outline, pixel_range, scale,
        }
        = self;
        re_export::CanvasItem::draw_msdf_texture_rect_region_full(surround_object, texture, rect, src_rect, modulate, outline, pixel_range, scale,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_lcd_texture_rect_region_ex`][super::CanvasItem::draw_lcd_texture_rect_region_ex]."]
#[must_use]
pub struct ExDrawLcdTextureRectRegion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, texture: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, rect: Rect2, src_rect: Rect2, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawLcdTextureRectRegion < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, rect: Rect2, src_rect: Rect2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), rect: rect, src_rect: src_rect, modulate: modulate,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, rect, src_rect, modulate,
        }
        = self;
        re_export::CanvasItem::draw_lcd_texture_rect_region_full(surround_object, texture, rect, src_rect, modulate,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_primitive_ex`][super::CanvasItem::draw_primitive_ex]."]
#[must_use]
pub struct ExDrawPrimitive < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, points: CowArg < 'a, PackedVector2Array >, colors: CowArg < 'a, PackedColorArray >, uvs: CowArg < 'a, PackedVector2Array >, texture: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawPrimitive < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: &'a PackedVector2Array, colors: &'a PackedColorArray, uvs: &'a PackedVector2Array,) -> Self {
        let texture = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), uvs: CowArg::Borrowed(uvs), texture: texture.into_arg(),
        }
    }
    #[inline]
    pub fn texture(self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a) -> Self {
        Self {
            texture: texture.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, colors, uvs, texture,
        }
        = self;
        re_export::CanvasItem::draw_primitive_full(surround_object, points.cow_as_arg(), colors.cow_as_arg(), uvs.cow_as_arg(), texture,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_polygon_ex`][super::CanvasItem::draw_polygon_ex]."]
#[must_use]
pub struct ExDrawPolygon < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, points: CowArg < 'a, PackedVector2Array >, colors: CowArg < 'a, PackedColorArray >, uvs: CowArg < 'a, PackedVector2Array >, texture: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawPolygon < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> Self {
        let uvs = PackedVector2Array::new();
        let texture = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), uvs: CowArg::Owned(uvs), texture: texture.into_arg(),
        }
    }
    #[inline]
    pub fn uvs(self, uvs: &'a PackedVector2Array) -> Self {
        Self {
            uvs: CowArg::Borrowed(uvs), .. self
        }
    }
    #[inline]
    pub fn texture(self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a) -> Self {
        Self {
            texture: texture.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, colors, uvs, texture,
        }
        = self;
        re_export::CanvasItem::draw_polygon_full(surround_object, points.cow_as_arg(), colors.cow_as_arg(), uvs.cow_as_arg(), texture,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_colored_polygon_ex`][super::CanvasItem::draw_colored_polygon_ex]."]
#[must_use]
pub struct ExDrawColoredPolygon < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, points: CowArg < 'a, PackedVector2Array >, color: Color, uvs: CowArg < 'a, PackedVector2Array >, texture: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawColoredPolygon < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, points: &'a PackedVector2Array, color: Color,) -> Self {
        let uvs = PackedVector2Array::new();
        let texture = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), color: color, uvs: CowArg::Owned(uvs), texture: texture.into_arg(),
        }
    }
    #[inline]
    pub fn uvs(self, uvs: &'a PackedVector2Array) -> Self {
        Self {
            uvs: CowArg::Borrowed(uvs), .. self
        }
    }
    #[inline]
    pub fn texture(self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a) -> Self {
        Self {
            texture: texture.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, color, uvs, texture,
        }
        = self;
        re_export::CanvasItem::draw_colored_polygon_full(surround_object, points.cow_as_arg(), color, uvs.cow_as_arg(), texture,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_string_ex`][super::CanvasItem::draw_string_ex]."]
#[must_use]
pub struct ExDrawString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::CanvasItem, font: CowArg < 'a, Option < Gd < crate::classes::Font > > >, pos: Vector2, text: CowArg < 'a, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawString < 'a > {
    fn new(surround_object: &'a re_export::CanvasItem, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, pos: Vector2, text: impl AsArg < GString > + 'a,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, modulate: modulate, justification_flags: justification_flags, direction: direction, orientation: orientation, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation, oversampling,
        }
        = self;
        re_export::CanvasItem::draw_string_full(surround_object, font, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation, oversampling,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_multiline_string_ex`][super::CanvasItem::draw_multiline_string_ex]."]
#[must_use]
pub struct ExDrawMultilineString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::CanvasItem, font: CowArg < 'a, Option < Gd < crate::classes::Font > > >, pos: Vector2, text: CowArg < 'a, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMultilineString < 'a > {
    fn new(surround_object: &'a re_export::CanvasItem, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, pos: Vector2, text: impl AsArg < GString > + 'a,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let max_lines = - 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let brk_flags = crate::obj::EngineBitfield::from_ord(3);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, max_lines: max_lines, modulate: modulate, brk_flags: brk_flags, justification_flags: justification_flags, direction: direction, orientation: orientation, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, max_lines: i32) -> Self {
        Self {
            max_lines: max_lines, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, brk_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: brk_flags, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation, oversampling,
        }
        = self;
        re_export::CanvasItem::draw_multiline_string_full(surround_object, font, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation, oversampling,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_string_outline_ex`][super::CanvasItem::draw_string_outline_ex]."]
#[must_use]
pub struct ExDrawStringOutline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::CanvasItem, font: CowArg < 'a, Option < Gd < crate::classes::Font > > >, pos: Vector2, text: CowArg < 'a, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawStringOutline < 'a > {
    fn new(surround_object: &'a re_export::CanvasItem, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, pos: Vector2, text: impl AsArg < GString > + 'a,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let size = 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, size: size, modulate: modulate, justification_flags: justification_flags, direction: direction, orientation: orientation, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation, oversampling,
        }
        = self;
        re_export::CanvasItem::draw_string_outline_full(surround_object, font, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation, oversampling,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_multiline_string_outline_ex`][super::CanvasItem::draw_multiline_string_outline_ex]."]
#[must_use]
pub struct ExDrawMultilineStringOutline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::CanvasItem, font: CowArg < 'a, Option < Gd < crate::classes::Font > > >, pos: Vector2, text: CowArg < 'a, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, size: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMultilineStringOutline < 'a > {
    fn new(surround_object: &'a re_export::CanvasItem, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, pos: Vector2, text: impl AsArg < GString > + 'a,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let max_lines = - 1i32;
        let size = 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let brk_flags = crate::obj::EngineBitfield::from_ord(3);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, max_lines: max_lines, size: size, modulate: modulate, brk_flags: brk_flags, justification_flags: justification_flags, direction: direction, orientation: orientation, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, max_lines: i32) -> Self {
        Self {
            max_lines: max_lines, .. self
        }
    }
    #[inline]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, brk_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: brk_flags, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation, oversampling,
        }
        = self;
        re_export::CanvasItem::draw_multiline_string_outline_full(surround_object, font, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation, oversampling,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_char_ex`][super::CanvasItem::draw_char_ex]."]
#[must_use]
pub struct ExDrawChar < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::CanvasItem, font: CowArg < 'a, Option < Gd < crate::classes::Font > > >, pos: Vector2, char: CowArg < 'a, GString >, font_size: i32, modulate: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawChar < 'a > {
    fn new(surround_object: &'a re_export::CanvasItem, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, pos: Vector2, char: impl AsArg < GString > + 'a,) -> Self {
        let font_size = 16i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), pos: pos, char: char.into_arg(), font_size: font_size, modulate: modulate, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, pos, char, font_size, modulate, oversampling,
        }
        = self;
        re_export::CanvasItem::draw_char_full(surround_object, font, pos, char, font_size, modulate, oversampling,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_char_outline_ex`][super::CanvasItem::draw_char_outline_ex]."]
#[must_use]
pub struct ExDrawCharOutline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::CanvasItem, font: CowArg < 'a, Option < Gd < crate::classes::Font > > >, pos: Vector2, char: CowArg < 'a, GString >, font_size: i32, size: i32, modulate: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawCharOutline < 'a > {
    fn new(surround_object: &'a re_export::CanvasItem, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, pos: Vector2, char: impl AsArg < GString > + 'a,) -> Self {
        let font_size = 16i32;
        let size = - 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), pos: pos, char: char.into_arg(), font_size: font_size, size: size, modulate: modulate, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, pos, char, font_size, size, modulate, oversampling,
        }
        = self;
        re_export::CanvasItem::draw_char_outline_full(surround_object, font, pos, char, font_size, size, modulate, oversampling,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_mesh_ex`][super::CanvasItem::draw_mesh_ex]."]
#[must_use]
pub struct ExDrawMesh < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, mesh: CowArg < 'a, Option < Gd < crate::classes::Mesh > > >, texture: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, transform: Transform2D, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMesh < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, mesh: impl AsArg < Option < Gd < crate::classes::Mesh >> > + 'a, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a,) -> Self {
        let transform = Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _);
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mesh: mesh.into_arg(), texture: texture.into_arg(), transform: transform, modulate: modulate,
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform2D) -> Self {
        Self {
            transform: transform, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, mesh, texture, transform, modulate,
        }
        = self;
        re_export::CanvasItem::draw_mesh_full(surround_object, mesh, texture, transform, modulate,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_set_transform_ex`][super::CanvasItem::draw_set_transform_ex]."]
#[must_use]
pub struct ExDrawSetTransform < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, position: Vector2, rotation: f32, scale: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawSetTransform < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, position: Vector2,) -> Self {
        let rotation = 0f32;
        let scale = Vector2::new(1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, rotation: rotation, scale: scale,
        }
    }
    #[inline]
    pub fn rotation(self, rotation: f32) -> Self {
        Self {
            rotation: rotation, .. self
        }
    }
    #[inline]
    pub fn scale(self, scale: Vector2) -> Self {
        Self {
            scale: scale, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, rotation, scale,
        }
        = self;
        re_export::CanvasItem::draw_set_transform_full(surround_object, position, rotation, scale,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_animation_slice_ex`][super::CanvasItem::draw_animation_slice_ex]."]
#[must_use]
pub struct ExDrawAnimationSlice < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CanvasItem, animation_length: f64, slice_begin: f64, slice_end: f64, offset: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawAnimationSlice < 'a > {
    fn new(surround_object: &'a mut re_export::CanvasItem, animation_length: f64, slice_begin: f64, slice_end: f64,) -> Self {
        let offset = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, animation_length: animation_length, slice_begin: slice_begin, slice_end: slice_end, offset: offset,
        }
    }
    #[inline]
    pub fn offset(self, offset: f64) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, animation_length, slice_begin, slice_end, offset,
        }
        = self;
        re_export::CanvasItem::draw_animation_slice_full(surround_object, animation_length, slice_begin, slice_end, offset,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureFilter {
    ord: i32
}
impl TextureFilter {
    #[doc(alias = "TEXTURE_FILTER_PARENT_NODE")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_PARENT_NODE`"]
    pub const PARENT_NODE: TextureFilter = TextureFilter {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_NEAREST`"]
    pub const NEAREST: TextureFilter = TextureFilter {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_LINEAR`"]
    pub const LINEAR: TextureFilter = TextureFilter {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_NEAREST_WITH_MIPMAPS`"]
    pub const NEAREST_WITH_MIPMAPS: TextureFilter = TextureFilter {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_LINEAR_WITH_MIPMAPS`"]
    pub const LINEAR_WITH_MIPMAPS: TextureFilter = TextureFilter {
        ord: 4i32
    };
    #[doc(alias = "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC`"]
    pub const NEAREST_WITH_MIPMAPS_ANISOTROPIC: TextureFilter = TextureFilter {
        ord: 5i32
    };
    #[doc(alias = "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC`"]
    pub const LINEAR_WITH_MIPMAPS_ANISOTROPIC: TextureFilter = TextureFilter {
        ord: 6i32
    };
    #[doc(alias = "TEXTURE_FILTER_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_MAX`"]
    pub const MAX: TextureFilter = TextureFilter {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for TextureFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureFilter {
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
            Self::PARENT_NODE => "PARENT_NODE", Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::NEAREST_WITH_MIPMAPS => "NEAREST_WITH_MIPMAPS", Self::LINEAR_WITH_MIPMAPS => "LINEAR_WITH_MIPMAPS", Self::NEAREST_WITH_MIPMAPS_ANISOTROPIC => "NEAREST_WITH_MIPMAPS_ANISOTROPIC", Self::LINEAR_WITH_MIPMAPS_ANISOTROPIC => "LINEAR_WITH_MIPMAPS_ANISOTROPIC", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureFilter::PARENT_NODE, TextureFilter::NEAREST, TextureFilter::LINEAR, TextureFilter::NEAREST_WITH_MIPMAPS, TextureFilter::LINEAR_WITH_MIPMAPS, TextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC, TextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PARENT_NODE", "TEXTURE_FILTER_PARENT_NODE", TextureFilter::PARENT_NODE), crate::meta::inspect::EnumConstant::new("NEAREST", "TEXTURE_FILTER_NEAREST", TextureFilter::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "TEXTURE_FILTER_LINEAR", TextureFilter::LINEAR), crate::meta::inspect::EnumConstant::new("NEAREST_WITH_MIPMAPS", "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS", TextureFilter::NEAREST_WITH_MIPMAPS), crate::meta::inspect::EnumConstant::new("LINEAR_WITH_MIPMAPS", "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS", TextureFilter::LINEAR_WITH_MIPMAPS), crate::meta::inspect::EnumConstant::new("NEAREST_WITH_MIPMAPS_ANISOTROPIC", "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC", TextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("LINEAR_WITH_MIPMAPS_ANISOTROPIC", "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC", TextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("MAX", "TEXTURE_FILTER_MAX", TextureFilter::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureFilter {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for TextureFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureRepeat {
    ord: i32
}
impl TextureRepeat {
    #[doc(alias = "TEXTURE_REPEAT_PARENT_NODE")]
    #[doc = "Godot enumerator name: `TEXTURE_REPEAT_PARENT_NODE`"]
    pub const PARENT_NODE: TextureRepeat = TextureRepeat {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_REPEAT_DISABLED")]
    #[doc = "Godot enumerator name: `TEXTURE_REPEAT_DISABLED`"]
    pub const DISABLED: TextureRepeat = TextureRepeat {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_REPEAT_ENABLED")]
    #[doc = "Godot enumerator name: `TEXTURE_REPEAT_ENABLED`"]
    pub const ENABLED: TextureRepeat = TextureRepeat {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_REPEAT_MIRROR")]
    #[doc = "Godot enumerator name: `TEXTURE_REPEAT_MIRROR`"]
    pub const MIRROR: TextureRepeat = TextureRepeat {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_REPEAT_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_REPEAT_MAX`"]
    pub const MAX: TextureRepeat = TextureRepeat {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for TextureRepeat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureRepeat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureRepeat {
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
            Self::PARENT_NODE => "PARENT_NODE", Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::MIRROR => "MIRROR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureRepeat::PARENT_NODE, TextureRepeat::DISABLED, TextureRepeat::ENABLED, TextureRepeat::MIRROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureRepeat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PARENT_NODE", "TEXTURE_REPEAT_PARENT_NODE", TextureRepeat::PARENT_NODE), crate::meta::inspect::EnumConstant::new("DISABLED", "TEXTURE_REPEAT_DISABLED", TextureRepeat::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "TEXTURE_REPEAT_ENABLED", TextureRepeat::ENABLED), crate::meta::inspect::EnumConstant::new("MIRROR", "TEXTURE_REPEAT_MIRROR", TextureRepeat::MIRROR), crate::meta::inspect::EnumConstant::new("MAX", "TEXTURE_REPEAT_MAX", TextureRepeat::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureRepeat {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for TextureRepeat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureRepeat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureRepeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ClipChildrenMode {
    ord: i32
}
impl ClipChildrenMode {
    #[doc(alias = "CLIP_CHILDREN_DISABLED")]
    #[doc = "Godot enumerator name: `CLIP_CHILDREN_DISABLED`"]
    pub const DISABLED: ClipChildrenMode = ClipChildrenMode {
        ord: 0i32
    };
    #[doc(alias = "CLIP_CHILDREN_ONLY")]
    #[doc = "Godot enumerator name: `CLIP_CHILDREN_ONLY`"]
    pub const ONLY: ClipChildrenMode = ClipChildrenMode {
        ord: 1i32
    };
    #[doc(alias = "CLIP_CHILDREN_AND_DRAW")]
    #[doc = "Godot enumerator name: `CLIP_CHILDREN_AND_DRAW`"]
    pub const AND_DRAW: ClipChildrenMode = ClipChildrenMode {
        ord: 2i32
    };
    #[doc(alias = "CLIP_CHILDREN_MAX")]
    #[doc = "Godot enumerator name: `CLIP_CHILDREN_MAX`"]
    pub const MAX: ClipChildrenMode = ClipChildrenMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ClipChildrenMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ClipChildrenMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ClipChildrenMode {
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
            Self::DISABLED => "DISABLED", Self::ONLY => "ONLY", Self::AND_DRAW => "AND_DRAW", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ClipChildrenMode::DISABLED, ClipChildrenMode::ONLY, ClipChildrenMode::AND_DRAW]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ClipChildrenMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "CLIP_CHILDREN_DISABLED", ClipChildrenMode::DISABLED), crate::meta::inspect::EnumConstant::new("ONLY", "CLIP_CHILDREN_ONLY", ClipChildrenMode::ONLY), crate::meta::inspect::EnumConstant::new("AND_DRAW", "CLIP_CHILDREN_AND_DRAW", ClipChildrenMode::AND_DRAW), crate::meta::inspect::EnumConstant::new("MAX", "CLIP_CHILDREN_MAX", ClipChildrenMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ClipChildrenMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ClipChildrenMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ClipChildrenMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ClipChildrenMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CanvasItem;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`CanvasItem`][crate::classes::CanvasItem] class."]
    pub struct SignalsOfCanvasItem < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfCanvasItem < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn draw(&mut self) -> SigDraw < 'c, C > {
            SigDraw {
                typed: TypedSignal::extract(&mut self.__internal_obj, "draw")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn visibility_changed(&mut self) -> SigVisibilityChanged < 'c, C > {
            SigVisibilityChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "visibility_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn hidden(&mut self) -> SigHidden < 'c, C > {
            SigHidden {
                typed: TypedSignal::extract(&mut self.__internal_obj, "hidden")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn item_rect_changed(&mut self) -> SigItemRectChanged < 'c, C > {
            SigItemRectChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "item_rect_changed")
            }
        }
    }
    type TypedSigDraw < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigDraw < 'c, C: WithSignals > {
        typed: TypedSigDraw < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDraw < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDraw < 'c, C > {
        type Target = TypedSigDraw < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDraw < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigVisibilityChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigVisibilityChanged < 'c, C: WithSignals > {
        typed: TypedSigVisibilityChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigVisibilityChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigVisibilityChanged < 'c, C > {
        type Target = TypedSigVisibilityChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigVisibilityChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigHidden < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigHidden < 'c, C: WithSignals > {
        typed: TypedSigHidden < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigHidden < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigHidden < 'c, C > {
        type Target = TypedSigHidden < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigHidden < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigItemRectChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigItemRectChanged < 'c, C: WithSignals > {
        typed: TypedSigItemRectChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemRectChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemRectChanged < 'c, C > {
        type Target = TypedSigItemRectChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemRectChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for CanvasItem {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCanvasItem < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfCanvasItem < 'c, C > {
        type Target = < < CanvasItem as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = CanvasItem;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfCanvasItem < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = CanvasItem;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}