#![doc = "Sidecar module for class [`NativeMenu`][crate::classes::NativeMenu].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `NativeMenu` enums](https://docs.godotengine.org/en/stable/classes/class_nativemenu.html#enumerations).\n\n"]
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
    #[doc = "Godot class `NativeMenu.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`native_menu`][crate::classes::native_menu]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `NativeMenu`](https://docs.godotengine.org/en/stable/classes/class_nativemenu.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct NativeMenu {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl NativeMenu {
        pub fn has_feature(&self, feature: crate::classes::native_menu::Feature,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::native_menu::Feature,);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5528usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "has_feature", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_system_menu(&self, menu_id: crate::classes::native_menu::SystemMenus,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::native_menu::SystemMenus,);
            let args = (menu_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5529usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "has_system_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_system_menu(&self, menu_id: crate::classes::native_menu::SystemMenus,) -> Rid {
            type CallRet = Rid;
            type CallParams = (crate::classes::native_menu::SystemMenus,);
            let args = (menu_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5530usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_system_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_system_menu_name(&self, menu_id: crate::classes::native_menu::SystemMenus,) -> GString {
            type CallRet = GString;
            type CallParams = (crate::classes::native_menu::SystemMenus,);
            let args = (menu_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5531usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_system_menu_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_menu(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5532usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "create_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_menu(&self, rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5533usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "has_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn free_menu(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5534usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "free_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self, rid: Rid,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5535usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn popup(&mut self, rid: Rid, position: Vector2i,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i,);
            let args = (rid, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5536usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "popup", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_interface_direction(&mut self, rid: Rid, is_rtl: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (rid, is_rtl,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5537usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_interface_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_popup_open_callback(&mut self, rid: Rid, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Callable >,);
            let args = (rid, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5538usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_popup_open_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_popup_open_callback(&self, rid: Rid,) -> Callable {
            type CallRet = Callable;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5539usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_popup_open_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_popup_close_callback(&mut self, rid: Rid, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Callable >,);
            let args = (rid, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5540usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_popup_close_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_popup_close_callback(&self, rid: Rid,) -> Callable {
            type CallRet = Callable;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5541usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_popup_close_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_minimum_width(&mut self, rid: Rid, width: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (rid, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5542usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_minimum_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_minimum_width(&self, rid: Rid,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_minimum_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_opened(&self, rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5544usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_opened", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_submenu_item_full(&mut self, rid: Rid, label: CowArg < GString >, submenu_rid: Rid, tag: RefArg < Variant >, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (Rid, CowArg < 'a0, GString >, Rid, RefArg < 'a1, Variant >, i32,);
            let args = (rid, label, submenu_rid, tag, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5545usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_submenu_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_submenu_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_submenu_item(&mut self, rid: Rid, label: impl AsArg < GString >, submenu_rid: Rid,) -> i32 {
            self.add_submenu_item_ex(rid, label, submenu_rid,) . done()
        }
        #[inline]
        pub fn add_submenu_item_ex < 'a > (&'a mut self, rid: Rid, label: impl AsArg < GString > + 'a, submenu_rid: Rid,) -> ExAddSubmenuItem < 'a > {
            ExAddSubmenuItem::new(self, rid, label, submenu_rid,)
        }
        pub(crate) fn add_item_full(&mut self, rid: Rid, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (Rid, CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32,);
            let args = (rid, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5546usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_item(&mut self, rid: Rid, label: impl AsArg < GString >,) -> i32 {
            self.add_item_ex(rid, label,) . done()
        }
        #[inline]
        pub fn add_item_ex < 'a > (&'a mut self, rid: Rid, label: impl AsArg < GString > + 'a,) -> ExAddItem < 'a > {
            ExAddItem::new(self, rid, label,)
        }
        pub(crate) fn add_check_item_full(&mut self, rid: Rid, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (Rid, CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32,);
            let args = (rid, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5547usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_check_item(&mut self, rid: Rid, label: impl AsArg < GString >,) -> i32 {
            self.add_check_item_ex(rid, label,) . done()
        }
        #[inline]
        pub fn add_check_item_ex < 'a > (&'a mut self, rid: Rid, label: impl AsArg < GString > + 'a,) -> ExAddCheckItem < 'a > {
            ExAddCheckItem::new(self, rid, label,)
        }
        pub(crate) fn add_icon_item_full(&mut self, rid: Rid, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (Rid, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (rid, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5548usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_icon_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_item(&mut self, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) -> i32 {
            self.add_icon_item_ex(rid, icon, label,) . done()
        }
        #[inline]
        pub fn add_icon_item_ex < 'a > (&'a mut self, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> ExAddIconItem < 'a > {
            ExAddIconItem::new(self, rid, icon, label,)
        }
        pub(crate) fn add_icon_check_item_full(&mut self, rid: Rid, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (Rid, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (rid, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5549usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_icon_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_check_item(&mut self, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) -> i32 {
            self.add_icon_check_item_ex(rid, icon, label,) . done()
        }
        #[inline]
        pub fn add_icon_check_item_ex < 'a > (&'a mut self, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> ExAddIconCheckItem < 'a > {
            ExAddIconCheckItem::new(self, rid, icon, label,)
        }
        pub(crate) fn add_radio_check_item_full(&mut self, rid: Rid, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (Rid, CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32,);
            let args = (rid, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5550usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_radio_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_radio_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_radio_check_item(&mut self, rid: Rid, label: impl AsArg < GString >,) -> i32 {
            self.add_radio_check_item_ex(rid, label,) . done()
        }
        #[inline]
        pub fn add_radio_check_item_ex < 'a > (&'a mut self, rid: Rid, label: impl AsArg < GString > + 'a,) -> ExAddRadioCheckItem < 'a > {
            ExAddRadioCheckItem::new(self, rid, label,)
        }
        pub(crate) fn add_icon_radio_check_item_full(&mut self, rid: Rid, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (Rid, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (rid, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5551usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_icon_radio_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_radio_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_radio_check_item(&mut self, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) -> i32 {
            self.add_icon_radio_check_item_ex(rid, icon, label,) . done()
        }
        #[inline]
        pub fn add_icon_radio_check_item_ex < 'a > (&'a mut self, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> ExAddIconRadioCheckItem < 'a > {
            ExAddIconRadioCheckItem::new(self, rid, icon, label,)
        }
        pub(crate) fn add_multistate_item_full(&mut self, rid: Rid, label: CowArg < GString >, max_states: i32, default_state: i32, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (Rid, CowArg < 'a0, GString >, i32, i32, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32,);
            let args = (rid, label, max_states, default_state, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5552usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_multistate_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_multistate_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_multistate_item(&mut self, rid: Rid, label: impl AsArg < GString >, max_states: i32, default_state: i32,) -> i32 {
            self.add_multistate_item_ex(rid, label, max_states, default_state,) . done()
        }
        #[inline]
        pub fn add_multistate_item_ex < 'a > (&'a mut self, rid: Rid, label: impl AsArg < GString > + 'a, max_states: i32, default_state: i32,) -> ExAddMultistateItem < 'a > {
            ExAddMultistateItem::new(self, rid, label, max_states, default_state,)
        }
        pub(crate) fn add_separator_full(&mut self, rid: Rid, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, i32,);
            let args = (rid, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5553usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_separator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_separator_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_separator(&mut self, rid: Rid,) -> i32 {
            self.add_separator_ex(rid,) . done()
        }
        #[inline]
        pub fn add_separator_ex < 'a > (&'a mut self, rid: Rid,) -> ExAddSeparator < 'a > {
            ExAddSeparator::new(self, rid,)
        }
        pub fn find_item_index_with_text(&self, rid: Rid, text: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (rid, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5554usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "find_item_index_with_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn find_item_index_with_tag(&self, rid: Rid, tag: &Variant,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >,);
            let args = (rid, RefArg::new(tag),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5555usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "find_item_index_with_tag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn find_item_index_with_submenu(&self, rid: Rid, submenu_rid: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, Rid,);
            let args = (rid, submenu_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5556usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "find_item_index_with_submenu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_item_checked(&self, rid: Rid, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5557usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_checked", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_item_checkable(&self, rid: Rid, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5558usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_checkable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_item_radio_checkable(&self, rid: Rid, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5559usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_radio_checkable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_callback(&self, rid: Rid, idx: i32,) -> Callable {
            type CallRet = Callable;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5560usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_key_callback(&self, rid: Rid, idx: i32,) -> Callable {
            type CallRet = Callable;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5561usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_key_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_tag(&self, rid: Rid, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5562usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_tag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_text(&self, rid: Rid, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5563usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_submenu(&self, rid: Rid, idx: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5564usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_submenu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_accelerator(&self, rid: Rid, idx: i32,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5565usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_accelerator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_item_disabled(&self, rid: Rid, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5566usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_item_hidden(&self, rid: Rid, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5567usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_tooltip(&self, rid: Rid, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5568usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_state(&self, rid: Rid, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5569usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_max_states(&self, rid: Rid, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5570usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_max_states", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_icon(&self, rid: Rid, idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5571usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_indentation_level(&self, rid: Rid, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5572usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_indentation_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_checked(&mut self, rid: Rid, idx: i32, checked: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (rid, idx, checked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5573usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_checked", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_checkable(&mut self, rid: Rid, idx: i32, checkable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (rid, idx, checkable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5574usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_checkable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_radio_checkable(&mut self, rid: Rid, idx: i32, checkable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (rid, idx, checkable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5575usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_radio_checkable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_callback(&mut self, rid: Rid, idx: i32, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, RefArg < 'a0, Callable >,);
            let args = (rid, idx, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5576usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_hover_callbacks(&mut self, rid: Rid, idx: i32, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, RefArg < 'a0, Callable >,);
            let args = (rid, idx, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5577usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_hover_callbacks", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_key_callback(&mut self, rid: Rid, idx: i32, key_callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, RefArg < 'a0, Callable >,);
            let args = (rid, idx, RefArg::new(key_callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5578usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_key_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_tag(&mut self, rid: Rid, idx: i32, tag: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, RefArg < 'a0, Variant >,);
            let args = (rid, idx, RefArg::new(tag),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5579usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_tag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_text(&mut self, rid: Rid, idx: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, CowArg < 'a0, GString >,);
            let args = (rid, idx, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5580usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_submenu(&mut self, rid: Rid, idx: i32, submenu_rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Rid,);
            let args = (rid, idx, submenu_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5581usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_submenu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_accelerator(&mut self, rid: Rid, idx: i32, keycode: crate::global::Key,) {
            type CallRet = ();
            type CallParams = (Rid, i32, crate::global::Key,);
            let args = (rid, idx, keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5582usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_accelerator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_disabled(&mut self, rid: Rid, idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (rid, idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5583usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_hidden(&mut self, rid: Rid, idx: i32, hidden: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (rid, idx, hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_tooltip(&mut self, rid: Rid, idx: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, CowArg < 'a0, GString >,);
            let args = (rid, idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_state(&mut self, rid: Rid, idx: i32, state: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (rid, idx, state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5586usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_max_states(&mut self, rid: Rid, idx: i32, max_states: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (rid, idx, max_states,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5587usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_max_states", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_icon(&mut self, rid: Rid, idx: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (rid, idx, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5588usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_indentation_level(&mut self, rid: Rid, idx: i32, level: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (rid, idx, level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_indentation_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_count(&self, rid: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_system_menu(&self, rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_system_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_item(&mut self, rid: Rid, idx: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "remove_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "clear", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for NativeMenu {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("NativeMenu"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for NativeMenu {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for NativeMenu {
        
    }
    impl crate::obj::Singleton for NativeMenu {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"NativeMenu"))
            }
        }
    }
    impl std::ops::Deref for NativeMenu {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for NativeMenu {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_NativeMenu__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `NativeMenu` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_submenu_item_ex`][super::NativeMenu::add_submenu_item_ex]."]
#[must_use]
pub struct ExAddSubmenuItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: CowArg < 'a, GString >, submenu_rid: Rid, tag: CowArg < 'a, Variant >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSubmenuItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'a, submenu_rid: Rid,) -> Self {
        let tag = Variant::nil();
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), submenu_rid: submenu_rid, tag: CowArg::Owned(tag), index: index,
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, rid, label, submenu_rid, tag, index,
        }
        = self;
        re_export::NativeMenu::add_submenu_item_full(surround_object, rid, label, submenu_rid, tag.cow_as_arg(), index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_item_ex`][super::NativeMenu::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, rid, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_item_full(surround_object, rid, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_check_item_ex`][super::NativeMenu::add_check_item_ex]."]
#[must_use]
pub struct ExAddCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, rid, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_check_item_full(surround_object, rid, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_icon_item_ex`][super::NativeMenu::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, icon: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, icon: icon.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, rid, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_icon_item_full(surround_object, rid, icon, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_icon_check_item_ex`][super::NativeMenu::add_icon_check_item_ex]."]
#[must_use]
pub struct ExAddIconCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, icon: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, icon: icon.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, rid, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_icon_check_item_full(surround_object, rid, icon, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_radio_check_item_ex`][super::NativeMenu::add_radio_check_item_ex]."]
#[must_use]
pub struct ExAddRadioCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, rid, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_radio_check_item_full(surround_object, rid, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_icon_radio_check_item_ex`][super::NativeMenu::add_icon_radio_check_item_ex]."]
#[must_use]
pub struct ExAddIconRadioCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, icon: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, icon: icon.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, rid, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_icon_radio_check_item_full(surround_object, rid, icon, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_multistate_item_ex`][super::NativeMenu::add_multistate_item_ex]."]
#[must_use]
pub struct ExAddMultistateItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: CowArg < 'a, GString >, max_states: i32, default_state: i32, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddMultistateItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'a, max_states: i32, default_state: i32,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), max_states: max_states, default_state: default_state, callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, rid, label, max_states, default_state, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_multistate_item_full(surround_object, rid, label, max_states, default_state, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_separator_ex`][super::NativeMenu::add_separator_ex]."]
#[must_use]
pub struct ExAddSeparator < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSeparator < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, rid, index,
        }
        = self;
        re_export::NativeMenu::add_separator_full(surround_object, rid, index,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Feature {
    ord: i32
}
impl Feature {
    #[doc(alias = "FEATURE_GLOBAL_MENU")]
    #[doc = "Godot enumerator name: `FEATURE_GLOBAL_MENU`"]
    pub const GLOBAL_MENU: Feature = Feature {
        ord: 0i32
    };
    #[doc(alias = "FEATURE_POPUP_MENU")]
    #[doc = "Godot enumerator name: `FEATURE_POPUP_MENU`"]
    pub const POPUP_MENU: Feature = Feature {
        ord: 1i32
    };
    #[doc(alias = "FEATURE_OPEN_CLOSE_CALLBACK")]
    #[doc = "Godot enumerator name: `FEATURE_OPEN_CLOSE_CALLBACK`"]
    pub const OPEN_CLOSE_CALLBACK: Feature = Feature {
        ord: 2i32
    };
    #[doc(alias = "FEATURE_HOVER_CALLBACK")]
    #[doc = "Godot enumerator name: `FEATURE_HOVER_CALLBACK`"]
    pub const HOVER_CALLBACK: Feature = Feature {
        ord: 3i32
    };
    #[doc(alias = "FEATURE_KEY_CALLBACK")]
    #[doc = "Godot enumerator name: `FEATURE_KEY_CALLBACK`"]
    pub const KEY_CALLBACK: Feature = Feature {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Feature") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Feature {
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
            Self::GLOBAL_MENU => "GLOBAL_MENU", Self::POPUP_MENU => "POPUP_MENU", Self::OPEN_CLOSE_CALLBACK => "OPEN_CLOSE_CALLBACK", Self::HOVER_CALLBACK => "HOVER_CALLBACK", Self::KEY_CALLBACK => "KEY_CALLBACK", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Feature::GLOBAL_MENU, Feature::POPUP_MENU, Feature::OPEN_CLOSE_CALLBACK, Feature::HOVER_CALLBACK, Feature::KEY_CALLBACK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Feature >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GLOBAL_MENU", "FEATURE_GLOBAL_MENU", Feature::GLOBAL_MENU), crate::meta::inspect::EnumConstant::new("POPUP_MENU", "FEATURE_POPUP_MENU", Feature::POPUP_MENU), crate::meta::inspect::EnumConstant::new("OPEN_CLOSE_CALLBACK", "FEATURE_OPEN_CLOSE_CALLBACK", Feature::OPEN_CLOSE_CALLBACK), crate::meta::inspect::EnumConstant::new("HOVER_CALLBACK", "FEATURE_HOVER_CALLBACK", Feature::HOVER_CALLBACK), crate::meta::inspect::EnumConstant::new("KEY_CALLBACK", "FEATURE_KEY_CALLBACK", Feature::KEY_CALLBACK)]
        }
    }
}
impl crate::meta::GodotConvert for Feature {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Feature {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Feature {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SystemMenus {
    ord: i32
}
impl SystemMenus {
    pub const INVALID_MENU_ID: SystemMenus = SystemMenus {
        ord: 0i32
    };
    pub const MAIN_MENU_ID: SystemMenus = SystemMenus {
        ord: 1i32
    };
    pub const APPLICATION_MENU_ID: SystemMenus = SystemMenus {
        ord: 2i32
    };
    pub const WINDOW_MENU_ID: SystemMenus = SystemMenus {
        ord: 3i32
    };
    pub const HELP_MENU_ID: SystemMenus = SystemMenus {
        ord: 4i32
    };
    pub const DOCK_MENU_ID: SystemMenus = SystemMenus {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for SystemMenus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SystemMenus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SystemMenus {
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
            Self::INVALID_MENU_ID => "INVALID_MENU_ID", Self::MAIN_MENU_ID => "MAIN_MENU_ID", Self::APPLICATION_MENU_ID => "APPLICATION_MENU_ID", Self::WINDOW_MENU_ID => "WINDOW_MENU_ID", Self::HELP_MENU_ID => "HELP_MENU_ID", Self::DOCK_MENU_ID => "DOCK_MENU_ID", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SystemMenus::INVALID_MENU_ID, SystemMenus::MAIN_MENU_ID, SystemMenus::APPLICATION_MENU_ID, SystemMenus::WINDOW_MENU_ID, SystemMenus::HELP_MENU_ID, SystemMenus::DOCK_MENU_ID]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SystemMenus >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INVALID_MENU_ID", "INVALID_MENU_ID", SystemMenus::INVALID_MENU_ID), crate::meta::inspect::EnumConstant::new("MAIN_MENU_ID", "MAIN_MENU_ID", SystemMenus::MAIN_MENU_ID), crate::meta::inspect::EnumConstant::new("APPLICATION_MENU_ID", "APPLICATION_MENU_ID", SystemMenus::APPLICATION_MENU_ID), crate::meta::inspect::EnumConstant::new("WINDOW_MENU_ID", "WINDOW_MENU_ID", SystemMenus::WINDOW_MENU_ID), crate::meta::inspect::EnumConstant::new("HELP_MENU_ID", "HELP_MENU_ID", SystemMenus::HELP_MENU_ID), crate::meta::inspect::EnumConstant::new("DOCK_MENU_ID", "DOCK_MENU_ID", SystemMenus::DOCK_MENU_ID)]
        }
    }
}
impl crate::meta::GodotConvert for SystemMenus {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SystemMenus {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SystemMenus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::NativeMenu;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for NativeMenu {
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