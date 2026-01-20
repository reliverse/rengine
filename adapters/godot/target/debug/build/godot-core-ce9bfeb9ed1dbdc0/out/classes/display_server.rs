#![doc = "Sidecar module for class [`DisplayServer`][crate::classes::DisplayServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `DisplayServer` enums](https://docs.godotengine.org/en/stable/classes/class_displayserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `DisplayServer.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`display_server`][crate::classes::display_server]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `DisplayServer`](https://docs.godotengine.org/en/stable/classes/class_displayserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct DisplayServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl DisplayServer {
        pub fn has_feature(&self, feature: crate::classes::display_server::Feature,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::display_server::Feature,);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(60usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "has_feature", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(61usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn help_set_search_callbacks(&mut self, search_callback: &Callable, action_callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Callable >, RefArg < 'a1, Callable >,);
            let args = (RefArg::new(search_callback), RefArg::new(action_callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(62usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "help_set_search_callbacks", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_popup_callbacks(&mut self, menu_root: impl AsArg < GString >, open_callback: &Callable, close_callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >,);
            let args = (menu_root.into_arg(), RefArg::new(open_callback), RefArg::new(close_callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(63usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_popup_callbacks", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn global_menu_add_submenu_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, submenu: CowArg < GString >, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, i32,);
            let args = (menu_root, label, submenu, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(64usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_submenu_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_submenu_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_submenu_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >, submenu: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_submenu_item_ex(menu_root, label, submenu,) . done()
        }
        #[inline]
        pub fn global_menu_add_submenu_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a, submenu: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddSubmenuItem < 'a > {
            ExGlobalMenuAddSubmenuItem::new(self, menu_root, label, submenu,)
        }
        pub(crate) fn global_menu_add_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (menu_root, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(65usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_item_ex(menu_root, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddItem < 'a > {
            ExGlobalMenuAddItem::new(self, menu_root, label,)
        }
        pub(crate) fn global_menu_add_check_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (menu_root, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(66usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_check_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_check_item_ex(menu_root, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_check_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddCheckItem < 'a > {
            ExGlobalMenuAddCheckItem::new(self, menu_root, label,)
        }
        pub(crate) fn global_menu_add_icon_item_full(&mut self, menu_root: CowArg < GString >, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, 'a5, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a2, GString >, RefArg < 'a3, Callable >, RefArg < 'a4, Callable >, RefArg < 'a5, Variant >, crate::global::Key, i32,);
            let args = (menu_root, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(67usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_icon_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_icon_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_icon_item(&mut self, menu_root: impl AsArg < GString >, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_icon_item_ex(menu_root, icon, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_icon_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddIconItem < 'a > {
            ExGlobalMenuAddIconItem::new(self, menu_root, icon, label,)
        }
        pub(crate) fn global_menu_add_icon_check_item_full(&mut self, menu_root: CowArg < GString >, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, 'a5, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a2, GString >, RefArg < 'a3, Callable >, RefArg < 'a4, Callable >, RefArg < 'a5, Variant >, crate::global::Key, i32,);
            let args = (menu_root, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(68usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_icon_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_icon_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_icon_check_item(&mut self, menu_root: impl AsArg < GString >, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_icon_check_item_ex(menu_root, icon, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_icon_check_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddIconCheckItem < 'a > {
            ExGlobalMenuAddIconCheckItem::new(self, menu_root, icon, label,)
        }
        pub(crate) fn global_menu_add_radio_check_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (menu_root, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(69usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_radio_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_radio_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_radio_check_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_radio_check_item_ex(menu_root, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_radio_check_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddRadioCheckItem < 'a > {
            ExGlobalMenuAddRadioCheckItem::new(self, menu_root, label,)
        }
        pub(crate) fn global_menu_add_icon_radio_check_item_full(&mut self, menu_root: CowArg < GString >, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, 'a5, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a2, GString >, RefArg < 'a3, Callable >, RefArg < 'a4, Callable >, RefArg < 'a5, Variant >, crate::global::Key, i32,);
            let args = (menu_root, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(70usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_icon_radio_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_icon_radio_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_icon_radio_check_item(&mut self, menu_root: impl AsArg < GString >, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_icon_radio_check_item_ex(menu_root, icon, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_icon_radio_check_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddIconRadioCheckItem < 'a > {
            ExGlobalMenuAddIconRadioCheckItem::new(self, menu_root, icon, label,)
        }
        pub(crate) fn global_menu_add_multistate_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, max_states: i32, default_state: i32, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32, i32, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (menu_root, label, max_states, default_state, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(71usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_multistate_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_multistate_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_multistate_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >, max_states: i32, default_state: i32,) -> i32 {
            self.global_menu_add_multistate_item_ex(menu_root, label, max_states, default_state,) . done()
        }
        #[inline]
        pub fn global_menu_add_multistate_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a, max_states: i32, default_state: i32,) -> ExGlobalMenuAddMultistateItem < 'a > {
            ExGlobalMenuAddMultistateItem::new(self, menu_root, label, max_states, default_state,)
        }
        pub(crate) fn global_menu_add_separator_full(&mut self, menu_root: CowArg < GString >, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(72usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_separator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_separator_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_separator(&mut self, menu_root: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_separator_ex(menu_root,) . done()
        }
        #[inline]
        pub fn global_menu_add_separator_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddSeparator < 'a > {
            ExGlobalMenuAddSeparator::new(self, menu_root,)
        }
        pub fn global_menu_get_item_index_from_text(&self, menu_root: impl AsArg < GString >, text: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (menu_root.into_arg(), text.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(73usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_index_from_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_item_index_from_tag(&self, menu_root: impl AsArg < GString >, tag: &Variant,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Variant >,);
            let args = (menu_root.into_arg(), RefArg::new(tag),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(74usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_index_from_tag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_is_item_checked(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(75usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_checked", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_is_item_checkable(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(76usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_checkable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_is_item_radio_checkable(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(77usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_radio_checkable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_item_callback(&self, menu_root: impl AsArg < GString >, idx: i32,) -> Callable {
            type CallRet = Callable;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(78usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_item_key_callback(&self, menu_root: impl AsArg < GString >, idx: i32,) -> Callable {
            type CallRet = Callable;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(79usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_key_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_item_tag(&self, menu_root: impl AsArg < GString >, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(80usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_tag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_item_text(&self, menu_root: impl AsArg < GString >, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(81usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_item_submenu(&self, menu_root: impl AsArg < GString >, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(82usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_submenu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_item_accelerator(&self, menu_root: impl AsArg < GString >, idx: i32,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(83usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_accelerator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_is_item_disabled(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(84usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_is_item_hidden(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(85usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_item_tooltip(&self, menu_root: impl AsArg < GString >, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(86usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_item_state(&self, menu_root: impl AsArg < GString >, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(87usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_item_max_states(&self, menu_root: impl AsArg < GString >, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(88usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_max_states", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_item_icon(&self, menu_root: impl AsArg < GString >, idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(89usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_item_indentation_level(&self, menu_root: impl AsArg < GString >, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(90usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_indentation_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_checked(&mut self, menu_root: impl AsArg < GString >, idx: i32, checked: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, bool,);
            let args = (menu_root.into_arg(), idx, checked,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(91usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_checked", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_checkable(&mut self, menu_root: impl AsArg < GString >, idx: i32, checkable: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, bool,);
            let args = (menu_root.into_arg(), idx, checkable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(92usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_checkable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_radio_checkable(&mut self, menu_root: impl AsArg < GString >, idx: i32, checkable: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, bool,);
            let args = (menu_root.into_arg(), idx, checkable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(93usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_radio_checkable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_callback(&mut self, menu_root: impl AsArg < GString >, idx: i32, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, RefArg < 'a1, Callable >,);
            let args = (menu_root.into_arg(), idx, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(94usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_hover_callbacks(&mut self, menu_root: impl AsArg < GString >, idx: i32, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, RefArg < 'a1, Callable >,);
            let args = (menu_root.into_arg(), idx, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(95usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_hover_callbacks", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_key_callback(&mut self, menu_root: impl AsArg < GString >, idx: i32, key_callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, RefArg < 'a1, Callable >,);
            let args = (menu_root.into_arg(), idx, RefArg::new(key_callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(96usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_key_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_tag(&mut self, menu_root: impl AsArg < GString >, idx: i32, tag: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, RefArg < 'a1, Variant >,);
            let args = (menu_root.into_arg(), idx, RefArg::new(tag),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(97usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_tag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_text(&mut self, menu_root: impl AsArg < GString >, idx: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, CowArg < 'a1, GString >,);
            let args = (menu_root.into_arg(), idx, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(98usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_submenu(&mut self, menu_root: impl AsArg < GString >, idx: i32, submenu: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, CowArg < 'a1, GString >,);
            let args = (menu_root.into_arg(), idx, submenu.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(99usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_submenu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_accelerator(&mut self, menu_root: impl AsArg < GString >, idx: i32, keycode: crate::global::Key,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, crate::global::Key,);
            let args = (menu_root.into_arg(), idx, keycode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_accelerator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_disabled(&mut self, menu_root: impl AsArg < GString >, idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, bool,);
            let args = (menu_root.into_arg(), idx, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_hidden(&mut self, menu_root: impl AsArg < GString >, idx: i32, hidden: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, bool,);
            let args = (menu_root.into_arg(), idx, hidden,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_tooltip(&mut self, menu_root: impl AsArg < GString >, idx: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, CowArg < 'a1, GString >,);
            let args = (menu_root.into_arg(), idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_state(&mut self, menu_root: impl AsArg < GString >, idx: i32, state: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32,);
            let args = (menu_root.into_arg(), idx, state,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_max_states(&mut self, menu_root: impl AsArg < GString >, idx: i32, max_states: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32,);
            let args = (menu_root.into_arg(), idx, max_states,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_max_states", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_icon(&mut self, menu_root: impl AsArg < GString >, idx: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (menu_root.into_arg(), idx, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_set_item_indentation_level(&mut self, menu_root: impl AsArg < GString >, idx: i32, level: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32,);
            let args = (menu_root.into_arg(), idx, level,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_indentation_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_item_count(&self, menu_root: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (menu_root.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_remove_item(&mut self, menu_root: impl AsArg < GString >, idx: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_remove_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_clear(&mut self, menu_root: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (menu_root.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_menu_get_system_menu_roots(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_system_menu_roots", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tts_is_speaking(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_is_speaking", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tts_is_paused(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_is_paused", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tts_get_voices(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_get_voices", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tts_get_voices_for_language(&self, language: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_get_voices_for_language", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn tts_speak_full(&mut self, text: CowArg < GString >, voice: CowArg < GString >, volume: i32, pitch: f32, rate: f32, utterance_id: i32, interrupt: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32, f32, f32, i32, bool,);
            let args = (text, voice, volume, pitch, rate, utterance_id, interrupt,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_speak", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::tts_speak_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn tts_speak(&mut self, text: impl AsArg < GString >, voice: impl AsArg < GString >,) {
            self.tts_speak_ex(text, voice,) . done()
        }
        #[inline]
        pub fn tts_speak_ex < 'a > (&'a mut self, text: impl AsArg < GString > + 'a, voice: impl AsArg < GString > + 'a,) -> ExTtsSpeak < 'a > {
            ExTtsSpeak::new(self, text, voice,)
        }
        pub fn tts_pause(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_pause", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tts_resume(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_resume", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tts_stop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_stop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tts_set_utterance_callback(&mut self, event: crate::classes::display_server::TtsUtteranceEvent, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::display_server::TtsUtteranceEvent, RefArg < 'a0, Callable >,);
            let args = (event, RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_set_utterance_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_dark_mode_supported(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "is_dark_mode_supported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_dark_mode(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "is_dark_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accent_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_accent_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_base_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_base_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_system_theme_change_callback(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "set_system_theme_change_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mouse_set_mode(&mut self, mouse_mode: crate::classes::display_server::MouseMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::MouseMode,);
            let args = (mouse_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "mouse_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mouse_get_mode(&self,) -> crate::classes::display_server::MouseMode {
            type CallRet = crate::classes::display_server::MouseMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "mouse_get_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn warp_mouse(&mut self, position: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "warp_mouse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mouse_get_position(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "mouse_get_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mouse_get_button_state(&self,) -> crate::global::MouseButtonMask {
            type CallRet = crate::global::MouseButtonMask;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "mouse_get_button_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clipboard_set(&mut self, clipboard: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (clipboard.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clipboard_get(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_get", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clipboard_get_image(&self,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_get_image", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clipboard_has(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_has", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clipboard_has_image(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_has_image", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clipboard_set_primary(&mut self, clipboard_primary: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (clipboard_primary.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_set_primary", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clipboard_get_primary(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_get_primary", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_display_cutouts(&self,) -> Array < Rect2 > {
            type CallRet = Array < Rect2 >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_display_cutouts", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_display_safe_area(&self,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_display_safe_area", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_screen_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_screen_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_primary_screen(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_primary_screen", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keyboard_focus_screen(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_keyboard_focus_screen", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_screen_from_rect(&self, rect: Rect2,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rect2,);
            let args = (rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_screen_from_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn screen_get_position_full(&self, screen: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_position(&self,) -> Vector2i {
            self.screen_get_position_ex() . done()
        }
        #[inline]
        pub fn screen_get_position_ex < 'a > (&'a self,) -> ExScreenGetPosition < 'a > {
            ExScreenGetPosition::new(self,)
        }
        pub(crate) fn screen_get_size_full(&self, screen: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_size(&self,) -> Vector2i {
            self.screen_get_size_ex() . done()
        }
        #[inline]
        pub fn screen_get_size_ex < 'a > (&'a self,) -> ExScreenGetSize < 'a > {
            ExScreenGetSize::new(self,)
        }
        pub(crate) fn screen_get_usable_rect_full(&self, screen: i32,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_usable_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_usable_rect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_usable_rect(&self,) -> Rect2i {
            self.screen_get_usable_rect_ex() . done()
        }
        #[inline]
        pub fn screen_get_usable_rect_ex < 'a > (&'a self,) -> ExScreenGetUsableRect < 'a > {
            ExScreenGetUsableRect::new(self,)
        }
        pub(crate) fn screen_get_dpi_full(&self, screen: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_dpi", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_dpi_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_dpi(&self,) -> i32 {
            self.screen_get_dpi_ex() . done()
        }
        #[inline]
        pub fn screen_get_dpi_ex < 'a > (&'a self,) -> ExScreenGetDpi < 'a > {
            ExScreenGetDpi::new(self,)
        }
        pub(crate) fn screen_get_scale_full(&self, screen: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_scale_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_scale(&self,) -> f32 {
            self.screen_get_scale_ex() . done()
        }
        #[inline]
        pub fn screen_get_scale_ex < 'a > (&'a self,) -> ExScreenGetScale < 'a > {
            ExScreenGetScale::new(self,)
        }
        pub fn is_touchscreen_available(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "is_touchscreen_available", Some(self.__validated_obj()), args,)
            }
        }
        pub fn screen_get_max_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_max_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn screen_get_refresh_rate_full(&self, screen: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_refresh_rate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_refresh_rate_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_refresh_rate(&self,) -> f32 {
            self.screen_get_refresh_rate_ex() . done()
        }
        #[inline]
        pub fn screen_get_refresh_rate_ex < 'a > (&'a self,) -> ExScreenGetRefreshRate < 'a > {
            ExScreenGetRefreshRate::new(self,)
        }
        pub fn screen_get_pixel(&self, position: Vector2i,) -> Color {
            type CallRet = Color;
            type CallParams = (Vector2i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_pixel", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn screen_get_image_full(&self, screen: i32,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_image_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_image(&self,) -> Option < Gd < crate::classes::Image > > {
            self.screen_get_image_ex() . done()
        }
        #[inline]
        pub fn screen_get_image_ex < 'a > (&'a self,) -> ExScreenGetImage < 'a > {
            ExScreenGetImage::new(self,)
        }
        pub fn screen_get_image_rect(&self, rect: Rect2i,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rect2i,);
            let args = (rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_image_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn screen_set_orientation_full(&mut self, orientation: crate::classes::display_server::ScreenOrientation, screen: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::ScreenOrientation, i32,);
            let args = (orientation, screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_set_orientation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_set_orientation_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_set_orientation(&mut self, orientation: crate::classes::display_server::ScreenOrientation,) {
            self.screen_set_orientation_ex(orientation,) . done()
        }
        #[inline]
        pub fn screen_set_orientation_ex < 'a > (&'a mut self, orientation: crate::classes::display_server::ScreenOrientation,) -> ExScreenSetOrientation < 'a > {
            ExScreenSetOrientation::new(self, orientation,)
        }
        pub(crate) fn screen_get_orientation_full(&self, screen: i32,) -> crate::classes::display_server::ScreenOrientation {
            type CallRet = crate::classes::display_server::ScreenOrientation;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_orientation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_orientation_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_orientation(&self,) -> crate::classes::display_server::ScreenOrientation {
            self.screen_get_orientation_ex() . done()
        }
        #[inline]
        pub fn screen_get_orientation_ex < 'a > (&'a self,) -> ExScreenGetOrientation < 'a > {
            ExScreenGetOrientation::new(self,)
        }
        pub fn screen_set_keep_on(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_set_keep_on", Some(self.__validated_obj()), args,)
            }
        }
        pub fn screen_is_kept_on(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_is_kept_on", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_window_list(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_window_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_window_at_screen_position(&self, position: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_window_at_screen_position", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn window_get_native_handle_full(&self, handle_type: crate::classes::display_server::HandleType, window_id: i32,) -> i64 {
            type CallRet = i64;
            type CallParams = (crate::classes::display_server::HandleType, i32,);
            let args = (handle_type, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_native_handle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_native_handle_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_native_handle(&self, handle_type: crate::classes::display_server::HandleType,) -> i64 {
            self.window_get_native_handle_ex(handle_type,) . done()
        }
        #[inline]
        pub fn window_get_native_handle_ex < 'a > (&'a self, handle_type: crate::classes::display_server::HandleType,) -> ExWindowGetNativeHandle < 'a > {
            ExWindowGetNativeHandle::new(self, handle_type,)
        }
        pub fn window_get_active_popup(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_active_popup", Some(self.__validated_obj()), args,)
            }
        }
        pub fn window_set_popup_safe_rect(&mut self, window: i32, rect: Rect2i,) {
            type CallRet = ();
            type CallParams = (i32, Rect2i,);
            let args = (window, rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_popup_safe_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn window_get_popup_safe_rect(&self, window: i32,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = (i32,);
            let args = (window,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_popup_safe_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn window_set_title_full(&mut self, title: CowArg < GString >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (title, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_title", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_title_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_title(&mut self, title: impl AsArg < GString >,) {
            self.window_set_title_ex(title,) . done()
        }
        #[inline]
        pub fn window_set_title_ex < 'a > (&'a mut self, title: impl AsArg < GString > + 'a,) -> ExWindowSetTitle < 'a > {
            ExWindowSetTitle::new(self, title,)
        }
        pub(crate) fn window_get_title_size_full(&self, title: CowArg < GString >, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (title, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_title_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_title_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_title_size(&self, title: impl AsArg < GString >,) -> Vector2i {
            self.window_get_title_size_ex(title,) . done()
        }
        #[inline]
        pub fn window_get_title_size_ex < 'a > (&'a self, title: impl AsArg < GString > + 'a,) -> ExWindowGetTitleSize < 'a > {
            ExWindowGetTitleSize::new(self, title,)
        }
        pub(crate) fn window_set_mouse_passthrough_full(&mut self, region: RefArg < PackedVector2Array >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >, i32,);
            let args = (region, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_mouse_passthrough", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_mouse_passthrough_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_mouse_passthrough(&mut self, region: &PackedVector2Array,) {
            self.window_set_mouse_passthrough_ex(region,) . done()
        }
        #[inline]
        pub fn window_set_mouse_passthrough_ex < 'a > (&'a mut self, region: &'a PackedVector2Array,) -> ExWindowSetMousePassthrough < 'a > {
            ExWindowSetMousePassthrough::new(self, region,)
        }
        pub(crate) fn window_get_current_screen_full(&self, window_id: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_current_screen", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_current_screen_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_current_screen(&self,) -> i32 {
            self.window_get_current_screen_ex() . done()
        }
        #[inline]
        pub fn window_get_current_screen_ex < 'a > (&'a self,) -> ExWindowGetCurrentScreen < 'a > {
            ExWindowGetCurrentScreen::new(self,)
        }
        pub(crate) fn window_set_current_screen_full(&mut self, screen: i32, window_id: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (screen, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_current_screen", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_current_screen_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_current_screen(&mut self, screen: i32,) {
            self.window_set_current_screen_ex(screen,) . done()
        }
        #[inline]
        pub fn window_set_current_screen_ex < 'a > (&'a mut self, screen: i32,) -> ExWindowSetCurrentScreen < 'a > {
            ExWindowSetCurrentScreen::new(self, screen,)
        }
        pub(crate) fn window_get_position_full(&self, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_position(&self,) -> Vector2i {
            self.window_get_position_ex() . done()
        }
        #[inline]
        pub fn window_get_position_ex < 'a > (&'a self,) -> ExWindowGetPosition < 'a > {
            ExWindowGetPosition::new(self,)
        }
        pub(crate) fn window_get_position_with_decorations_full(&self, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_position_with_decorations", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_position_with_decorations_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_position_with_decorations(&self,) -> Vector2i {
            self.window_get_position_with_decorations_ex() . done()
        }
        #[inline]
        pub fn window_get_position_with_decorations_ex < 'a > (&'a self,) -> ExWindowGetPositionWithDecorations < 'a > {
            ExWindowGetPositionWithDecorations::new(self,)
        }
        pub(crate) fn window_set_position_full(&mut self, position: Vector2i, window_id: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (position, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_position(&mut self, position: Vector2i,) {
            self.window_set_position_ex(position,) . done()
        }
        #[inline]
        pub fn window_set_position_ex < 'a > (&'a mut self, position: Vector2i,) -> ExWindowSetPosition < 'a > {
            ExWindowSetPosition::new(self, position,)
        }
        pub(crate) fn window_get_size_full(&self, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_size(&self,) -> Vector2i {
            self.window_get_size_ex() . done()
        }
        #[inline]
        pub fn window_get_size_ex < 'a > (&'a self,) -> ExWindowGetSize < 'a > {
            ExWindowGetSize::new(self,)
        }
        pub(crate) fn window_set_size_full(&mut self, size: Vector2i, window_id: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (size, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_size(&mut self, size: Vector2i,) {
            self.window_set_size_ex(size,) . done()
        }
        #[inline]
        pub fn window_set_size_ex < 'a > (&'a mut self, size: Vector2i,) -> ExWindowSetSize < 'a > {
            ExWindowSetSize::new(self, size,)
        }
        pub(crate) fn window_set_rect_changed_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i32,);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_rect_changed_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_rect_changed_callback_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_rect_changed_callback(&mut self, callback: &Callable,) {
            self.window_set_rect_changed_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_rect_changed_callback_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExWindowSetRectChangedCallback < 'a > {
            ExWindowSetRectChangedCallback::new(self, callback,)
        }
        pub(crate) fn window_set_window_event_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i32,);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_window_event_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_window_event_callback_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_window_event_callback(&mut self, callback: &Callable,) {
            self.window_set_window_event_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_window_event_callback_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExWindowSetWindowEventCallback < 'a > {
            ExWindowSetWindowEventCallback::new(self, callback,)
        }
        pub(crate) fn window_set_input_event_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i32,);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_input_event_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_input_event_callback_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_input_event_callback(&mut self, callback: &Callable,) {
            self.window_set_input_event_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_input_event_callback_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExWindowSetInputEventCallback < 'a > {
            ExWindowSetInputEventCallback::new(self, callback,)
        }
        pub(crate) fn window_set_input_text_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i32,);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_input_text_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_input_text_callback_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_input_text_callback(&mut self, callback: &Callable,) {
            self.window_set_input_text_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_input_text_callback_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExWindowSetInputTextCallback < 'a > {
            ExWindowSetInputTextCallback::new(self, callback,)
        }
        pub(crate) fn window_set_drop_files_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i32,);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_drop_files_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_drop_files_callback_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_drop_files_callback(&mut self, callback: &Callable,) {
            self.window_set_drop_files_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_drop_files_callback_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExWindowSetDropFilesCallback < 'a > {
            ExWindowSetDropFilesCallback::new(self, callback,)
        }
        pub(crate) fn window_get_attached_instance_id_full(&self, window_id: i32,) -> u64 {
            type CallRet = u64;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_attached_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_attached_instance_id_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_attached_instance_id(&self,) -> u64 {
            self.window_get_attached_instance_id_ex() . done()
        }
        #[inline]
        pub fn window_get_attached_instance_id_ex < 'a > (&'a self,) -> ExWindowGetAttachedInstanceId < 'a > {
            ExWindowGetAttachedInstanceId::new(self,)
        }
        pub(crate) fn window_get_max_size_full(&self, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_max_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_max_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_max_size(&self,) -> Vector2i {
            self.window_get_max_size_ex() . done()
        }
        #[inline]
        pub fn window_get_max_size_ex < 'a > (&'a self,) -> ExWindowGetMaxSize < 'a > {
            ExWindowGetMaxSize::new(self,)
        }
        pub(crate) fn window_set_max_size_full(&mut self, max_size: Vector2i, window_id: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (max_size, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_max_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_max_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_max_size(&mut self, max_size: Vector2i,) {
            self.window_set_max_size_ex(max_size,) . done()
        }
        #[inline]
        pub fn window_set_max_size_ex < 'a > (&'a mut self, max_size: Vector2i,) -> ExWindowSetMaxSize < 'a > {
            ExWindowSetMaxSize::new(self, max_size,)
        }
        pub(crate) fn window_get_min_size_full(&self, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_min_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_min_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_min_size(&self,) -> Vector2i {
            self.window_get_min_size_ex() . done()
        }
        #[inline]
        pub fn window_get_min_size_ex < 'a > (&'a self,) -> ExWindowGetMinSize < 'a > {
            ExWindowGetMinSize::new(self,)
        }
        pub(crate) fn window_set_min_size_full(&mut self, min_size: Vector2i, window_id: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (min_size, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_min_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_min_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_min_size(&mut self, min_size: Vector2i,) {
            self.window_set_min_size_ex(min_size,) . done()
        }
        #[inline]
        pub fn window_set_min_size_ex < 'a > (&'a mut self, min_size: Vector2i,) -> ExWindowSetMinSize < 'a > {
            ExWindowSetMinSize::new(self, min_size,)
        }
        pub(crate) fn window_get_size_with_decorations_full(&self, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_size_with_decorations", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_size_with_decorations_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_size_with_decorations(&self,) -> Vector2i {
            self.window_get_size_with_decorations_ex() . done()
        }
        #[inline]
        pub fn window_get_size_with_decorations_ex < 'a > (&'a self,) -> ExWindowGetSizeWithDecorations < 'a > {
            ExWindowGetSizeWithDecorations::new(self,)
        }
        pub(crate) fn window_get_mode_full(&self, window_id: i32,) -> crate::classes::display_server::WindowMode {
            type CallRet = crate::classes::display_server::WindowMode;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_mode_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_mode(&self,) -> crate::classes::display_server::WindowMode {
            self.window_get_mode_ex() . done()
        }
        #[inline]
        pub fn window_get_mode_ex < 'a > (&'a self,) -> ExWindowGetMode < 'a > {
            ExWindowGetMode::new(self,)
        }
        pub(crate) fn window_set_mode_full(&mut self, mode: crate::classes::display_server::WindowMode, window_id: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::WindowMode, i32,);
            let args = (mode, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_mode_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_mode(&mut self, mode: crate::classes::display_server::WindowMode,) {
            self.window_set_mode_ex(mode,) . done()
        }
        #[inline]
        pub fn window_set_mode_ex < 'a > (&'a mut self, mode: crate::classes::display_server::WindowMode,) -> ExWindowSetMode < 'a > {
            ExWindowSetMode::new(self, mode,)
        }
        pub(crate) fn window_set_flag_full(&mut self, flag: crate::classes::display_server::WindowFlags, enabled: bool, window_id: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::WindowFlags, bool, i32,);
            let args = (flag, enabled, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_flag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_flag_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_flag(&mut self, flag: crate::classes::display_server::WindowFlags, enabled: bool,) {
            self.window_set_flag_ex(flag, enabled,) . done()
        }
        #[inline]
        pub fn window_set_flag_ex < 'a > (&'a mut self, flag: crate::classes::display_server::WindowFlags, enabled: bool,) -> ExWindowSetFlag < 'a > {
            ExWindowSetFlag::new(self, flag, enabled,)
        }
        pub(crate) fn window_get_flag_full(&self, flag: crate::classes::display_server::WindowFlags, window_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::display_server::WindowFlags, i32,);
            let args = (flag, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_flag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_flag_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_flag(&self, flag: crate::classes::display_server::WindowFlags,) -> bool {
            self.window_get_flag_ex(flag,) . done()
        }
        #[inline]
        pub fn window_get_flag_ex < 'a > (&'a self, flag: crate::classes::display_server::WindowFlags,) -> ExWindowGetFlag < 'a > {
            ExWindowGetFlag::new(self, flag,)
        }
        pub(crate) fn window_set_window_buttons_offset_full(&mut self, offset: Vector2i, window_id: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (offset, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_window_buttons_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_window_buttons_offset_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_window_buttons_offset(&mut self, offset: Vector2i,) {
            self.window_set_window_buttons_offset_ex(offset,) . done()
        }
        #[inline]
        pub fn window_set_window_buttons_offset_ex < 'a > (&'a mut self, offset: Vector2i,) -> ExWindowSetWindowButtonsOffset < 'a > {
            ExWindowSetWindowButtonsOffset::new(self, offset,)
        }
        pub(crate) fn window_get_safe_title_margins_full(&self, window_id: i32,) -> Vector3i {
            type CallRet = Vector3i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_safe_title_margins", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_safe_title_margins_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_safe_title_margins(&self,) -> Vector3i {
            self.window_get_safe_title_margins_ex() . done()
        }
        #[inline]
        pub fn window_get_safe_title_margins_ex < 'a > (&'a self,) -> ExWindowGetSafeTitleMargins < 'a > {
            ExWindowGetSafeTitleMargins::new(self,)
        }
        pub(crate) fn window_request_attention_full(&mut self, window_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_request_attention", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_request_attention_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_request_attention(&mut self,) {
            self.window_request_attention_ex() . done()
        }
        #[inline]
        pub fn window_request_attention_ex < 'a > (&'a mut self,) -> ExWindowRequestAttention < 'a > {
            ExWindowRequestAttention::new(self,)
        }
        pub(crate) fn window_move_to_foreground_full(&mut self, window_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_move_to_foreground", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_move_to_foreground_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_move_to_foreground(&mut self,) {
            self.window_move_to_foreground_ex() . done()
        }
        #[inline]
        pub fn window_move_to_foreground_ex < 'a > (&'a mut self,) -> ExWindowMoveToForeground < 'a > {
            ExWindowMoveToForeground::new(self,)
        }
        pub(crate) fn window_is_focused_full(&self, window_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_is_focused", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_is_focused_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_is_focused(&self,) -> bool {
            self.window_is_focused_ex() . done()
        }
        #[inline]
        pub fn window_is_focused_ex < 'a > (&'a self,) -> ExWindowIsFocused < 'a > {
            ExWindowIsFocused::new(self,)
        }
        pub(crate) fn window_can_draw_full(&self, window_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_can_draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_can_draw_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_can_draw(&self,) -> bool {
            self.window_can_draw_ex() . done()
        }
        #[inline]
        pub fn window_can_draw_ex < 'a > (&'a self,) -> ExWindowCanDraw < 'a > {
            ExWindowCanDraw::new(self,)
        }
        pub fn window_set_transient(&mut self, window_id: i32, parent_window_id: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (window_id, parent_window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_transient", Some(self.__validated_obj()), args,)
            }
        }
        pub fn window_set_exclusive(&mut self, window_id: i32, exclusive: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (window_id, exclusive,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_exclusive", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn window_set_ime_active_full(&mut self, active: bool, window_id: i32,) {
            type CallRet = ();
            type CallParams = (bool, i32,);
            let args = (active, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_ime_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_ime_active_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_ime_active(&mut self, active: bool,) {
            self.window_set_ime_active_ex(active,) . done()
        }
        #[inline]
        pub fn window_set_ime_active_ex < 'a > (&'a mut self, active: bool,) -> ExWindowSetImeActive < 'a > {
            ExWindowSetImeActive::new(self, active,)
        }
        pub(crate) fn window_set_ime_position_full(&mut self, position: Vector2i, window_id: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (position, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_ime_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_ime_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_ime_position(&mut self, position: Vector2i,) {
            self.window_set_ime_position_ex(position,) . done()
        }
        #[inline]
        pub fn window_set_ime_position_ex < 'a > (&'a mut self, position: Vector2i,) -> ExWindowSetImePosition < 'a > {
            ExWindowSetImePosition::new(self, position,)
        }
        pub(crate) fn window_set_vsync_mode_full(&mut self, vsync_mode: crate::classes::display_server::VSyncMode, window_id: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::VSyncMode, i32,);
            let args = (vsync_mode, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_vsync_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_vsync_mode_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_vsync_mode(&mut self, vsync_mode: crate::classes::display_server::VSyncMode,) {
            self.window_set_vsync_mode_ex(vsync_mode,) . done()
        }
        #[inline]
        pub fn window_set_vsync_mode_ex < 'a > (&'a mut self, vsync_mode: crate::classes::display_server::VSyncMode,) -> ExWindowSetVSyncMode < 'a > {
            ExWindowSetVSyncMode::new(self, vsync_mode,)
        }
        pub(crate) fn window_get_vsync_mode_full(&self, window_id: i32,) -> crate::classes::display_server::VSyncMode {
            type CallRet = crate::classes::display_server::VSyncMode;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_vsync_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_vsync_mode_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_vsync_mode(&self,) -> crate::classes::display_server::VSyncMode {
            self.window_get_vsync_mode_ex() . done()
        }
        #[inline]
        pub fn window_get_vsync_mode_ex < 'a > (&'a self,) -> ExWindowGetVSyncMode < 'a > {
            ExWindowGetVSyncMode::new(self,)
        }
        pub(crate) fn window_is_maximize_allowed_full(&self, window_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_is_maximize_allowed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_is_maximize_allowed_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_is_maximize_allowed(&self,) -> bool {
            self.window_is_maximize_allowed_ex() . done()
        }
        #[inline]
        pub fn window_is_maximize_allowed_ex < 'a > (&'a self,) -> ExWindowIsMaximizeAllowed < 'a > {
            ExWindowIsMaximizeAllowed::new(self,)
        }
        pub fn window_maximize_on_title_dbl_click(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_maximize_on_title_dbl_click", Some(self.__validated_obj()), args,)
            }
        }
        pub fn window_minimize_on_title_dbl_click(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_minimize_on_title_dbl_click", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn window_start_drag_full(&mut self, window_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_start_drag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_start_drag_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_start_drag(&mut self,) {
            self.window_start_drag_ex() . done()
        }
        #[inline]
        pub fn window_start_drag_ex < 'a > (&'a mut self,) -> ExWindowStartDrag < 'a > {
            ExWindowStartDrag::new(self,)
        }
        pub(crate) fn window_start_resize_full(&mut self, edge: crate::classes::display_server::WindowResizeEdge, window_id: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::WindowResizeEdge, i32,);
            let args = (edge, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_start_resize", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_start_resize_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_start_resize(&mut self, edge: crate::classes::display_server::WindowResizeEdge,) {
            self.window_start_resize_ex(edge,) . done()
        }
        #[inline]
        pub fn window_start_resize_ex < 'a > (&'a mut self, edge: crate::classes::display_server::WindowResizeEdge,) -> ExWindowStartResize < 'a > {
            ExWindowStartResize::new(self, edge,)
        }
        pub fn accessibility_should_increase_contrast(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_should_increase_contrast", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_should_reduce_animation(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_should_reduce_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_should_reduce_transparency(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_should_reduce_transparency", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_screen_reader_active(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_screen_reader_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_create_element(&mut self, window_id: i32, role: crate::classes::display_server::AccessibilityRole,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32, crate::classes::display_server::AccessibilityRole,);
            let args = (window_id, role,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_create_element", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn accessibility_create_sub_element_full(&mut self, parent_rid: Rid, role: crate::classes::display_server::AccessibilityRole, insert_pos: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, crate::classes::display_server::AccessibilityRole, i32,);
            let args = (parent_rid, role, insert_pos,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_create_sub_element", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::accessibility_create_sub_element_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn accessibility_create_sub_element(&mut self, parent_rid: Rid, role: crate::classes::display_server::AccessibilityRole,) -> Rid {
            self.accessibility_create_sub_element_ex(parent_rid, role,) . done()
        }
        #[inline]
        pub fn accessibility_create_sub_element_ex < 'a > (&'a mut self, parent_rid: Rid, role: crate::classes::display_server::AccessibilityRole,) -> ExAccessibilityCreateSubElement < 'a > {
            ExAccessibilityCreateSubElement::new(self, parent_rid, role,)
        }
        pub(crate) fn accessibility_create_sub_text_edit_elements_full(&mut self, parent_rid: Rid, shaped_text: Rid, min_height: f32, insert_pos: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, Rid, f32, i32,);
            let args = (parent_rid, shaped_text, min_height, insert_pos,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_create_sub_text_edit_elements", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::accessibility_create_sub_text_edit_elements_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn accessibility_create_sub_text_edit_elements(&mut self, parent_rid: Rid, shaped_text: Rid, min_height: f32,) -> Rid {
            self.accessibility_create_sub_text_edit_elements_ex(parent_rid, shaped_text, min_height,) . done()
        }
        #[inline]
        pub fn accessibility_create_sub_text_edit_elements_ex < 'a > (&'a mut self, parent_rid: Rid, shaped_text: Rid, min_height: f32,) -> ExAccessibilityCreateSubTextEditElements < 'a > {
            ExAccessibilityCreateSubTextEditElements::new(self, parent_rid, shaped_text, min_height,)
        }
        pub fn accessibility_has_element(&self, id: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_has_element", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_free_element(&mut self, id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_free_element", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_element_set_meta(&mut self, id: Rid, meta: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >,);
            let args = (id, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_element_set_meta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_element_get_meta(&self, id: Rid,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_element_get_meta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_set_window_rect(&mut self, window_id: i32, rect_out: Rect2, rect_in: Rect2,) {
            type CallRet = ();
            type CallParams = (i32, Rect2, Rect2,);
            let args = (window_id, rect_out, rect_in,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_set_window_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_set_window_focused(&mut self, window_id: i32, focused: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (window_id, focused,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_set_window_focused", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_focus(&mut self, id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_get_window_root(&self, window_id: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_get_window_root", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_role(&mut self, id: Rid, role: crate::classes::display_server::AccessibilityRole,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::display_server::AccessibilityRole,);
            let args = (id, role,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_role", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_name(&mut self, id: Rid, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_extra_info(&mut self, id: Rid, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_extra_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_description(&mut self, id: Rid, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, description.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_value(&mut self, id: Rid, value: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, value.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_tooltip(&mut self, id: Rid, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_bounds(&mut self, id: Rid, p_rect: Rect2,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2,);
            let args = (id, p_rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_bounds", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_transform(&mut self, id: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (id, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_add_child(&mut self, id: Rid, child_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, child_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_child", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_add_related_controls(&mut self, id: Rid, related_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, related_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_related_controls", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_add_related_details(&mut self, id: Rid, related_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, related_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_related_details", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_add_related_described_by(&mut self, id: Rid, related_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, related_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_related_described_by", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_add_related_flow_to(&mut self, id: Rid, related_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, related_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_related_flow_to", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_add_related_labeled_by(&mut self, id: Rid, related_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, related_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_related_labeled_by", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_add_related_radio_group(&mut self, id: Rid, related_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, related_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_related_radio_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_active_descendant(&mut self, id: Rid, other_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, other_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_active_descendant", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_next_on_line(&mut self, id: Rid, other_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, other_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_next_on_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_previous_on_line(&mut self, id: Rid, other_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, other_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_previous_on_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_member_of(&mut self, id: Rid, group_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, group_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_member_of", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_in_page_link_target(&mut self, id: Rid, other_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, other_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_in_page_link_target", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_error_message(&mut self, id: Rid, other_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, other_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_error_message", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_live(&mut self, id: Rid, live: crate::classes::display_server::AccessibilityLiveMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::display_server::AccessibilityLiveMode,);
            let args = (id, live,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_live", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_add_action(&mut self, id: Rid, action: crate::classes::display_server::AccessibilityAction, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, crate::classes::display_server::AccessibilityAction, RefArg < 'a0, Callable >,);
            let args = (id, action, RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_action", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_add_custom_action(&mut self, id: Rid, action_id: i32, action_description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, CowArg < 'a0, GString >,);
            let args = (id, action_id, action_description.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_custom_action", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_table_row_count(&mut self, id: Rid, count: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_table_row_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_table_column_count(&mut self, id: Rid, count: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_table_column_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_table_row_index(&mut self, id: Rid, index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_table_row_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_table_column_index(&mut self, id: Rid, index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_table_column_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_table_cell_position(&mut self, id: Rid, row_index: i32, column_index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (id, row_index, column_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_table_cell_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_table_cell_span(&mut self, id: Rid, row_span: i32, column_span: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (id, row_span, column_span,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_table_cell_span", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_list_item_count(&mut self, id: Rid, size: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_list_item_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_list_item_index(&mut self, id: Rid, index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_list_item_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_list_item_level(&mut self, id: Rid, level: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, level,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_list_item_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_list_item_selected(&mut self, id: Rid, selected: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (id, selected,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_list_item_selected", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_list_item_expanded(&mut self, id: Rid, expanded: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (id, expanded,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_list_item_expanded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_popup_type(&mut self, id: Rid, popup: crate::classes::display_server::AccessibilityPopupType,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::display_server::AccessibilityPopupType,);
            let args = (id, popup,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_popup_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_checked(&mut self, id: Rid, checekd: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (id, checekd,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_checked", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_num_value(&mut self, id: Rid, position: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (id, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_num_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_num_range(&mut self, id: Rid, min: f64, max: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64, f64,);
            let args = (id, min, max,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_num_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_num_step(&mut self, id: Rid, step: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (id, step,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_num_step", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_num_jump(&mut self, id: Rid, jump: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (id, jump,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_num_jump", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_scroll_x(&mut self, id: Rid, position: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (id, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_scroll_x", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_scroll_x_range(&mut self, id: Rid, min: f64, max: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64, f64,);
            let args = (id, min, max,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_scroll_x_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_scroll_y(&mut self, id: Rid, position: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (id, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_scroll_y", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_scroll_y_range(&mut self, id: Rid, min: f64, max: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64, f64,);
            let args = (id, min, max,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_scroll_y_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_text_decorations(&mut self, id: Rid, underline: bool, strikethrough: bool, overline: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool, bool, bool,);
            let args = (id, underline, strikethrough, overline,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_text_decorations", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_text_align(&mut self, id: Rid, align: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (Rid, crate::global::HorizontalAlignment,);
            let args = (id, align,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_text_align", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_text_selection(&mut self, id: Rid, text_start_id: Rid, start_char: i32, text_end_id: Rid, end_char: i32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, i32, Rid, i32,);
            let args = (id, text_start_id, start_char, text_end_id, end_char,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_text_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_flag(&mut self, id: Rid, flag: crate::classes::display_server::AccessibilityFlags, value: bool,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::display_server::AccessibilityFlags, bool,);
            let args = (id, flag, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_classname(&mut self, id: Rid, classname: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, classname.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_classname", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_placeholder(&mut self, id: Rid, placeholder: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, placeholder.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_language(&mut self, id: Rid, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_text_orientation(&mut self, id: Rid, vertical: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (id, vertical,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_text_orientation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_list_orientation(&mut self, id: Rid, vertical: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (id, vertical,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_list_orientation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_shortcut(&mut self, id: Rid, shortcut: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, shortcut.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_url(&mut self, id: Rid, url: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, url.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_url", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_role_description(&mut self, id: Rid, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, description.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_role_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_state_description(&mut self, id: Rid, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, description.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_state_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_color_value(&mut self, id: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (id, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_color_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_background_color(&mut self, id: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (id, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_background_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accessibility_update_set_foreground_color(&mut self, id: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (id, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_foreground_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn ime_get_selection(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "ime_get_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn ime_get_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "ime_get_text", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn virtual_keyboard_show_full(&mut self, existing_text: CowArg < GString >, position: Rect2, type_: crate::classes::display_server::VirtualKeyboardType, max_length: i32, cursor_start: i32, cursor_end: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, Rect2, crate::classes::display_server::VirtualKeyboardType, i32, i32, i32,);
            let args = (existing_text, position, type_, max_length, cursor_start, cursor_end,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "virtual_keyboard_show", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::virtual_keyboard_show_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn virtual_keyboard_show(&mut self, existing_text: impl AsArg < GString >,) {
            self.virtual_keyboard_show_ex(existing_text,) . done()
        }
        #[inline]
        pub fn virtual_keyboard_show_ex < 'a > (&'a mut self, existing_text: impl AsArg < GString > + 'a,) -> ExVirtualKeyboardShow < 'a > {
            ExVirtualKeyboardShow::new(self, existing_text,)
        }
        pub fn virtual_keyboard_hide(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "virtual_keyboard_hide", Some(self.__validated_obj()), args,)
            }
        }
        pub fn virtual_keyboard_get_height(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "virtual_keyboard_get_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_hardware_keyboard(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "has_hardware_keyboard", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hardware_keyboard_connection_change_callback(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "set_hardware_keyboard_connection_change_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn cursor_set_shape(&mut self, shape: crate::classes::display_server::CursorShape,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::CursorShape,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "cursor_set_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn cursor_get_shape(&self,) -> crate::classes::display_server::CursorShape {
            type CallRet = crate::classes::display_server::CursorShape;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "cursor_get_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn cursor_set_custom_image_full(&mut self, cursor: CowArg < Option < Gd < crate::classes::Resource > > >, shape: crate::classes::display_server::CursorShape, hotspot: Vector2,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource > > >, crate::classes::display_server::CursorShape, Vector2,);
            let args = (cursor, shape, hotspot,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "cursor_set_custom_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::cursor_set_custom_image_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn cursor_set_custom_image(&mut self, cursor: impl AsArg < Option < Gd < crate::classes::Resource >> >,) {
            self.cursor_set_custom_image_ex(cursor,) . done()
        }
        #[inline]
        pub fn cursor_set_custom_image_ex < 'a > (&'a mut self, cursor: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'a,) -> ExCursorSetCustomImage < 'a > {
            ExCursorSetCustomImage::new(self, cursor,)
        }
        pub fn get_swap_cancel_ok(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_swap_cancel_ok", Some(self.__validated_obj()), args,)
            }
        }
        pub fn enable_for_stealing_focus(&mut self, process_id: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (process_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "enable_for_stealing_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn dialog_show(&mut self, title: impl AsArg < GString >, description: impl AsArg < GString >, buttons: &PackedStringArray, callback: &Callable,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, PackedStringArray >, RefArg < 'a3, Callable >,);
            let args = (title.into_arg(), description.into_arg(), RefArg::new(buttons), RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "dialog_show", Some(self.__validated_obj()), args,)
            }
        }
        pub fn dialog_input_text(&mut self, title: impl AsArg < GString >, description: impl AsArg < GString >, existing_text: impl AsArg < GString >, callback: &Callable,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, RefArg < 'a3, Callable >,);
            let args = (title.into_arg(), description.into_arg(), existing_text.into_arg(), RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "dialog_input_text", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn file_dialog_show_full(&mut self, title: CowArg < GString >, current_directory: CowArg < GString >, filename: CowArg < GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: RefArg < PackedStringArray >, callback: RefArg < Callable >, parent_window_id: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, bool, crate::classes::display_server::FileDialogMode, RefArg < 'a3, PackedStringArray >, RefArg < 'a4, Callable >, i32,);
            let args = (title, current_directory, filename, show_hidden, mode, filters, callback, parent_window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "file_dialog_show", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::file_dialog_show_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn file_dialog_show(&mut self, title: impl AsArg < GString >, current_directory: impl AsArg < GString >, filename: impl AsArg < GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &PackedStringArray, callback: &Callable,) -> crate::global::Error {
            self.file_dialog_show_ex(title, current_directory, filename, show_hidden, mode, filters, callback,) . done()
        }
        #[inline]
        pub fn file_dialog_show_ex < 'a > (&'a mut self, title: impl AsArg < GString > + 'a, current_directory: impl AsArg < GString > + 'a, filename: impl AsArg < GString > + 'a, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &'a PackedStringArray, callback: &'a Callable,) -> ExFileDialogShow < 'a > {
            ExFileDialogShow::new(self, title, current_directory, filename, show_hidden, mode, filters, callback,)
        }
        pub(crate) fn file_dialog_with_options_show_full(&mut self, title: CowArg < GString >, current_directory: CowArg < GString >, root: CowArg < GString >, filename: CowArg < GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: RefArg < PackedStringArray >, options: RefArg < Array < VarDictionary > >, callback: RefArg < Callable >, parent_window_id: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, 'a5, 'a6, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, CowArg < 'a3, GString >, bool, crate::classes::display_server::FileDialogMode, RefArg < 'a4, PackedStringArray >, RefArg < 'a5, Array < VarDictionary > >, RefArg < 'a6, Callable >, i32,);
            let args = (title, current_directory, root, filename, show_hidden, mode, filters, options, callback, parent_window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "file_dialog_with_options_show", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::file_dialog_with_options_show_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn file_dialog_with_options_show(&mut self, title: impl AsArg < GString >, current_directory: impl AsArg < GString >, root: impl AsArg < GString >, filename: impl AsArg < GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &PackedStringArray, options: &Array < VarDictionary >, callback: &Callable,) -> crate::global::Error {
            self.file_dialog_with_options_show_ex(title, current_directory, root, filename, show_hidden, mode, filters, options, callback,) . done()
        }
        #[inline]
        pub fn file_dialog_with_options_show_ex < 'a > (&'a mut self, title: impl AsArg < GString > + 'a, current_directory: impl AsArg < GString > + 'a, root: impl AsArg < GString > + 'a, filename: impl AsArg < GString > + 'a, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &'a PackedStringArray, options: &'a Array < VarDictionary >, callback: &'a Callable,) -> ExFileDialogWithOptionsShow < 'a > {
            ExFileDialogWithOptionsShow::new(self, title, current_directory, root, filename, show_hidden, mode, filters, options, callback,)
        }
        pub fn beep(&self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "beep", Some(self.__validated_obj()), args,)
            }
        }
        pub fn keyboard_get_layout_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_layout_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn keyboard_get_current_layout(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_current_layout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn keyboard_set_current_layout(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_set_current_layout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn keyboard_get_layout_language(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_layout_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn keyboard_get_layout_name(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_layout_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn keyboard_get_keycode_from_physical(&self, keycode: crate::global::Key,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams = (crate::global::Key,);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_keycode_from_physical", Some(self.__validated_obj()), args,)
            }
        }
        pub fn keyboard_get_label_from_physical(&self, keycode: crate::global::Key,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams = (crate::global::Key,);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_label_from_physical", Some(self.__validated_obj()), args,)
            }
        }
        pub fn show_emoji_and_symbol_picker(&self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "show_emoji_and_symbol_picker", Some(self.__validated_obj()), args,)
            }
        }
        pub fn color_picker(&mut self, callback: &Callable,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "color_picker", Some(self.__validated_obj()), args,)
            }
        }
        pub fn process_events(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "process_events", Some(self.__validated_obj()), args,)
            }
        }
        pub fn force_process_and_drop_events(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "force_process_and_drop_events", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_native_icon(&mut self, filename: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (filename.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "set_native_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_icon(&mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >,);
            let args = (image.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "set_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_status_indicator(&mut self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, tooltip: impl AsArg < GString >, callback: &Callable,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >,);
            let args = (icon.into_arg(), tooltip.into_arg(), RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "create_status_indicator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn status_indicator_set_icon(&mut self, id: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (id, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_set_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn status_indicator_set_tooltip(&mut self, id: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (id, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_set_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn status_indicator_set_menu(&mut self, id: i32, menu_rid: Rid,) {
            type CallRet = ();
            type CallParams = (i32, Rid,);
            let args = (id, menu_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_set_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn status_indicator_set_callback(&mut self, id: i32, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Callable >,);
            let args = (id, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_set_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn status_indicator_get_rect(&self, id: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_get_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn delete_status_indicator(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "delete_status_indicator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tablet_get_driver_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tablet_get_driver_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tablet_get_driver_name(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tablet_get_driver_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tablet_get_current_driver(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tablet_get_current_driver", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tablet_set_current_driver(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tablet_set_current_driver", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_window_transparency_available(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "is_window_transparency_available", Some(self.__validated_obj()), args,)
            }
        }
        pub fn register_additional_output(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (object.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "register_additional_output", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unregister_additional_output(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (object.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "unregister_additional_output", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_additional_outputs(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "has_additional_outputs", Some(self.__validated_obj()), args,)
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
        pub const INVALID_SCREEN: i32 = - 1i32;
        pub const SCREEN_WITH_MOUSE_FOCUS: i32 = - 4i32;
        pub const SCREEN_WITH_KEYBOARD_FOCUS: i32 = - 3i32;
        pub const SCREEN_PRIMARY: i32 = - 2i32;
        pub const SCREEN_OF_MAIN_WINDOW: i32 = - 1i32;
        pub const MAIN_WINDOW_ID: i32 = 0i32;
        pub const INVALID_WINDOW_ID: i32 = - 1i32;
        pub const INVALID_INDICATOR_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for DisplayServer {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("DisplayServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for DisplayServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for DisplayServer {
        
    }
    impl crate::obj::Singleton for DisplayServer {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"DisplayServer"))
            }
        }
    }
    impl std::ops::Deref for DisplayServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for DisplayServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_DisplayServer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `DisplayServer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_submenu_item_ex`][super::DisplayServer::global_menu_add_submenu_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddSubmenuItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, label: CowArg < 'a, GString >, submenu: CowArg < 'a, GString >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddSubmenuItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a, submenu: impl AsArg < GString > + 'a,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), submenu: submenu.into_arg(), index: index,
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
            _phantom, surround_object, menu_root, label, submenu, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_submenu_item_full(surround_object, menu_root, label, submenu, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_item_ex`][super::DisplayServer::global_menu_add_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, menu_root, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_item_full(surround_object, menu_root, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_check_item_ex`][super::DisplayServer::global_menu_add_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, menu_root, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_check_item_full(surround_object, menu_root, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_icon_item_ex`][super::DisplayServer::global_menu_add_icon_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddIconItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, icon: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddIconItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), icon: icon.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, menu_root, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_icon_item_full(surround_object, menu_root, icon, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_icon_check_item_ex`][super::DisplayServer::global_menu_add_icon_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddIconCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, icon: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddIconCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), icon: icon.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, menu_root, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_icon_check_item_full(surround_object, menu_root, icon, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_radio_check_item_ex`][super::DisplayServer::global_menu_add_radio_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddRadioCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, menu_root, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_radio_check_item_full(surround_object, menu_root, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_icon_radio_check_item_ex`][super::DisplayServer::global_menu_add_icon_radio_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddIconRadioCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, icon: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddIconRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), icon: icon.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, menu_root, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_icon_radio_check_item_full(surround_object, menu_root, icon, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_multistate_item_ex`][super::DisplayServer::global_menu_add_multistate_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddMultistateItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, label: CowArg < 'a, GString >, max_states: i32, default_state: i32, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddMultistateItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a, max_states: i32, default_state: i32,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), max_states: max_states, default_state: default_state, callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, menu_root, label, max_states, default_state, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_multistate_item_full(surround_object, menu_root, label, max_states, default_state, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_separator_ex`][super::DisplayServer::global_menu_add_separator_ex]."]
#[must_use]
pub struct ExGlobalMenuAddSeparator < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddSeparator < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), index: index,
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
            _phantom, surround_object, menu_root, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_separator_full(surround_object, menu_root, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::tts_speak_ex`][super::DisplayServer::tts_speak_ex]."]
#[must_use]
pub struct ExTtsSpeak < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, text: CowArg < 'a, GString >, voice: CowArg < 'a, GString >, volume: i32, pitch: f32, rate: f32, utterance_id: i32, interrupt: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTtsSpeak < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, text: impl AsArg < GString > + 'a, voice: impl AsArg < GString > + 'a,) -> Self {
        let volume = 50i32;
        let pitch = 1f32;
        let rate = 1f32;
        let utterance_id = 0i32;
        let interrupt = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), voice: voice.into_arg(), volume: volume, pitch: pitch, rate: rate, utterance_id: utterance_id, interrupt: interrupt,
        }
    }
    #[inline]
    pub fn volume(self, volume: i32) -> Self {
        Self {
            volume: volume, .. self
        }
    }
    #[inline]
    pub fn pitch(self, pitch: f32) -> Self {
        Self {
            pitch: pitch, .. self
        }
    }
    #[inline]
    pub fn rate(self, rate: f32) -> Self {
        Self {
            rate: rate, .. self
        }
    }
    #[inline]
    pub fn utterance_id(self, utterance_id: i32) -> Self {
        Self {
            utterance_id: utterance_id, .. self
        }
    }
    #[inline]
    pub fn interrupt(self, interrupt: bool) -> Self {
        Self {
            interrupt: interrupt, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, text, voice, volume, pitch, rate, utterance_id, interrupt,
        }
        = self;
        re_export::DisplayServer::tts_speak_full(surround_object, text, voice, volume, pitch, rate, utterance_id, interrupt,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_position_ex`][super::DisplayServer::screen_get_position_ex]."]
#[must_use]
pub struct ExScreenGetPosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetPosition < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_position_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_size_ex`][super::DisplayServer::screen_get_size_ex]."]
#[must_use]
pub struct ExScreenGetSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_size_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_usable_rect_ex`][super::DisplayServer::screen_get_usable_rect_ex]."]
#[must_use]
pub struct ExScreenGetUsableRect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetUsableRect < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2i {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_usable_rect_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_dpi_ex`][super::DisplayServer::screen_get_dpi_ex]."]
#[must_use]
pub struct ExScreenGetDpi < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetDpi < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_dpi_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_scale_ex`][super::DisplayServer::screen_get_scale_ex]."]
#[must_use]
pub struct ExScreenGetScale < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetScale < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_scale_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_refresh_rate_ex`][super::DisplayServer::screen_get_refresh_rate_ex]."]
#[must_use]
pub struct ExScreenGetRefreshRate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetRefreshRate < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_refresh_rate_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_image_ex`][super::DisplayServer::screen_get_image_ex]."]
#[must_use]
pub struct ExScreenGetImage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetImage < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Image > > {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_image_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_set_orientation_ex`][super::DisplayServer::screen_set_orientation_ex]."]
#[must_use]
pub struct ExScreenSetOrientation < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, orientation: crate::classes::display_server::ScreenOrientation, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenSetOrientation < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, orientation: crate::classes::display_server::ScreenOrientation,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, orientation: orientation, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, orientation, screen,
        }
        = self;
        re_export::DisplayServer::screen_set_orientation_full(surround_object, orientation, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_orientation_ex`][super::DisplayServer::screen_get_orientation_ex]."]
#[must_use]
pub struct ExScreenGetOrientation < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetOrientation < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::display_server::ScreenOrientation {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_orientation_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_native_handle_ex`][super::DisplayServer::window_get_native_handle_ex]."]
#[must_use]
pub struct ExWindowGetNativeHandle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, handle_type: crate::classes::display_server::HandleType, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetNativeHandle < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer, handle_type: crate::classes::display_server::HandleType,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, handle_type: handle_type, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, handle_type, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_native_handle_full(surround_object, handle_type, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_title_ex`][super::DisplayServer::window_set_title_ex]."]
#[must_use]
pub struct ExWindowSetTitle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, title: CowArg < 'a, GString >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetTitle < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, title: impl AsArg < GString > + 'a,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, title: title.into_arg(), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, title, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_title_full(surround_object, title, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_title_size_ex`][super::DisplayServer::window_get_title_size_ex]."]
#[must_use]
pub struct ExWindowGetTitleSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, title: CowArg < 'a, GString >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetTitleSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer, title: impl AsArg < GString > + 'a,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, title: title.into_arg(), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, title, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_title_size_full(surround_object, title, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_mouse_passthrough_ex`][super::DisplayServer::window_set_mouse_passthrough_ex]."]
#[must_use]
pub struct ExWindowSetMousePassthrough < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, region: CowArg < 'a, PackedVector2Array >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetMousePassthrough < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, region: &'a PackedVector2Array,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, region: CowArg::Borrowed(region), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, region, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_mouse_passthrough_full(surround_object, region.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_current_screen_ex`][super::DisplayServer::window_get_current_screen_ex]."]
#[must_use]
pub struct ExWindowGetCurrentScreen < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetCurrentScreen < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_current_screen_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_current_screen_ex`][super::DisplayServer::window_set_current_screen_ex]."]
#[must_use]
pub struct ExWindowSetCurrentScreen < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, screen: i32, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetCurrentScreen < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, screen: i32,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, screen, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_current_screen_full(surround_object, screen, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_position_ex`][super::DisplayServer::window_get_position_ex]."]
#[must_use]
pub struct ExWindowGetPosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetPosition < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_position_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_position_with_decorations_ex`][super::DisplayServer::window_get_position_with_decorations_ex]."]
#[must_use]
pub struct ExWindowGetPositionWithDecorations < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetPositionWithDecorations < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_position_with_decorations_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_position_ex`][super::DisplayServer::window_set_position_ex]."]
#[must_use]
pub struct ExWindowSetPosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, position: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetPosition < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, position: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_position_full(surround_object, position, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_size_ex`][super::DisplayServer::window_get_size_ex]."]
#[must_use]
pub struct ExWindowGetSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_size_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_size_ex`][super::DisplayServer::window_set_size_ex]."]
#[must_use]
pub struct ExWindowSetSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, size: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetSize < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, size: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size: size, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, size, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_size_full(surround_object, size, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_rect_changed_callback_ex`][super::DisplayServer::window_set_rect_changed_callback_ex]."]
#[must_use]
pub struct ExWindowSetRectChangedCallback < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, callback: CowArg < 'a, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetRectChangedCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: &'a Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_rect_changed_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_window_event_callback_ex`][super::DisplayServer::window_set_window_event_callback_ex]."]
#[must_use]
pub struct ExWindowSetWindowEventCallback < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, callback: CowArg < 'a, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetWindowEventCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: &'a Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_window_event_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_input_event_callback_ex`][super::DisplayServer::window_set_input_event_callback_ex]."]
#[must_use]
pub struct ExWindowSetInputEventCallback < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, callback: CowArg < 'a, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetInputEventCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: &'a Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_input_event_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_input_text_callback_ex`][super::DisplayServer::window_set_input_text_callback_ex]."]
#[must_use]
pub struct ExWindowSetInputTextCallback < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, callback: CowArg < 'a, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetInputTextCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: &'a Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_input_text_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_drop_files_callback_ex`][super::DisplayServer::window_set_drop_files_callback_ex]."]
#[must_use]
pub struct ExWindowSetDropFilesCallback < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, callback: CowArg < 'a, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetDropFilesCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: &'a Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_drop_files_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_attached_instance_id_ex`][super::DisplayServer::window_get_attached_instance_id_ex]."]
#[must_use]
pub struct ExWindowGetAttachedInstanceId < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetAttachedInstanceId < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> u64 {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_attached_instance_id_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_max_size_ex`][super::DisplayServer::window_get_max_size_ex]."]
#[must_use]
pub struct ExWindowGetMaxSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetMaxSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_max_size_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_max_size_ex`][super::DisplayServer::window_set_max_size_ex]."]
#[must_use]
pub struct ExWindowSetMaxSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, max_size: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetMaxSize < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, max_size: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, max_size: max_size, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, max_size, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_max_size_full(surround_object, max_size, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_min_size_ex`][super::DisplayServer::window_get_min_size_ex]."]
#[must_use]
pub struct ExWindowGetMinSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetMinSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_min_size_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_min_size_ex`][super::DisplayServer::window_set_min_size_ex]."]
#[must_use]
pub struct ExWindowSetMinSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, min_size: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetMinSize < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, min_size: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, min_size: min_size, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, min_size, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_min_size_full(surround_object, min_size, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_size_with_decorations_ex`][super::DisplayServer::window_get_size_with_decorations_ex]."]
#[must_use]
pub struct ExWindowGetSizeWithDecorations < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetSizeWithDecorations < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_size_with_decorations_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_mode_ex`][super::DisplayServer::window_get_mode_ex]."]
#[must_use]
pub struct ExWindowGetMode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetMode < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::display_server::WindowMode {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_mode_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_mode_ex`][super::DisplayServer::window_set_mode_ex]."]
#[must_use]
pub struct ExWindowSetMode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, mode: crate::classes::display_server::WindowMode, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetMode < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, mode: crate::classes::display_server::WindowMode,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mode: mode, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, mode, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_mode_full(surround_object, mode, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_flag_ex`][super::DisplayServer::window_set_flag_ex]."]
#[must_use]
pub struct ExWindowSetFlag < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, flag: crate::classes::display_server::WindowFlags, enabled: bool, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetFlag < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, flag: crate::classes::display_server::WindowFlags, enabled: bool,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, flag: flag, enabled: enabled, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, flag, enabled, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_flag_full(surround_object, flag, enabled, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_flag_ex`][super::DisplayServer::window_get_flag_ex]."]
#[must_use]
pub struct ExWindowGetFlag < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, flag: crate::classes::display_server::WindowFlags, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetFlag < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer, flag: crate::classes::display_server::WindowFlags,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, flag: flag, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, flag, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_flag_full(surround_object, flag, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_window_buttons_offset_ex`][super::DisplayServer::window_set_window_buttons_offset_ex]."]
#[must_use]
pub struct ExWindowSetWindowButtonsOffset < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, offset: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetWindowButtonsOffset < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, offset: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, offset: offset, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, offset, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_window_buttons_offset_full(surround_object, offset, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_safe_title_margins_ex`][super::DisplayServer::window_get_safe_title_margins_ex]."]
#[must_use]
pub struct ExWindowGetSafeTitleMargins < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetSafeTitleMargins < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_safe_title_margins_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_request_attention_ex`][super::DisplayServer::window_request_attention_ex]."]
#[must_use]
pub struct ExWindowRequestAttention < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowRequestAttention < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_request_attention_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_move_to_foreground_ex`][super::DisplayServer::window_move_to_foreground_ex]."]
#[must_use]
pub struct ExWindowMoveToForeground < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowMoveToForeground < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_move_to_foreground_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_is_focused_ex`][super::DisplayServer::window_is_focused_ex]."]
#[must_use]
pub struct ExWindowIsFocused < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowIsFocused < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_is_focused_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_can_draw_ex`][super::DisplayServer::window_can_draw_ex]."]
#[must_use]
pub struct ExWindowCanDraw < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowCanDraw < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_can_draw_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_ime_active_ex`][super::DisplayServer::window_set_ime_active_ex]."]
#[must_use]
pub struct ExWindowSetImeActive < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, active: bool, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetImeActive < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, active: bool,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, active: active, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, active, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_ime_active_full(surround_object, active, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_ime_position_ex`][super::DisplayServer::window_set_ime_position_ex]."]
#[must_use]
pub struct ExWindowSetImePosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, position: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetImePosition < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, position: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_ime_position_full(surround_object, position, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_vsync_mode_ex`][super::DisplayServer::window_set_vsync_mode_ex]."]
#[must_use]
pub struct ExWindowSetVSyncMode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, vsync_mode: crate::classes::display_server::VSyncMode, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetVSyncMode < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, vsync_mode: crate::classes::display_server::VSyncMode,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, vsync_mode: vsync_mode, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, vsync_mode, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_vsync_mode_full(surround_object, vsync_mode, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_vsync_mode_ex`][super::DisplayServer::window_get_vsync_mode_ex]."]
#[must_use]
pub struct ExWindowGetVSyncMode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetVSyncMode < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::display_server::VSyncMode {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_vsync_mode_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_is_maximize_allowed_ex`][super::DisplayServer::window_is_maximize_allowed_ex]."]
#[must_use]
pub struct ExWindowIsMaximizeAllowed < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowIsMaximizeAllowed < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_is_maximize_allowed_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_start_drag_ex`][super::DisplayServer::window_start_drag_ex]."]
#[must_use]
pub struct ExWindowStartDrag < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowStartDrag < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_start_drag_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_start_resize_ex`][super::DisplayServer::window_start_resize_ex]."]
#[must_use]
pub struct ExWindowStartResize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, edge: crate::classes::display_server::WindowResizeEdge, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowStartResize < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, edge: crate::classes::display_server::WindowResizeEdge,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, edge: edge, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, edge, window_id,
        }
        = self;
        re_export::DisplayServer::window_start_resize_full(surround_object, edge, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::accessibility_create_sub_element_ex`][super::DisplayServer::accessibility_create_sub_element_ex]."]
#[must_use]
pub struct ExAccessibilityCreateSubElement < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, parent_rid: Rid, role: crate::classes::display_server::AccessibilityRole, insert_pos: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAccessibilityCreateSubElement < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, parent_rid: Rid, role: crate::classes::display_server::AccessibilityRole,) -> Self {
        let insert_pos = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parent_rid: parent_rid, role: role, insert_pos: insert_pos,
        }
    }
    #[inline]
    pub fn insert_pos(self, insert_pos: i32) -> Self {
        Self {
            insert_pos: insert_pos, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, parent_rid, role, insert_pos,
        }
        = self;
        re_export::DisplayServer::accessibility_create_sub_element_full(surround_object, parent_rid, role, insert_pos,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::accessibility_create_sub_text_edit_elements_ex`][super::DisplayServer::accessibility_create_sub_text_edit_elements_ex]."]
#[must_use]
pub struct ExAccessibilityCreateSubTextEditElements < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, parent_rid: Rid, shaped_text: Rid, min_height: f32, insert_pos: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAccessibilityCreateSubTextEditElements < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, parent_rid: Rid, shaped_text: Rid, min_height: f32,) -> Self {
        let insert_pos = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parent_rid: parent_rid, shaped_text: shaped_text, min_height: min_height, insert_pos: insert_pos,
        }
    }
    #[inline]
    pub fn insert_pos(self, insert_pos: i32) -> Self {
        Self {
            insert_pos: insert_pos, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, parent_rid, shaped_text, min_height, insert_pos,
        }
        = self;
        re_export::DisplayServer::accessibility_create_sub_text_edit_elements_full(surround_object, parent_rid, shaped_text, min_height, insert_pos,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::virtual_keyboard_show_ex`][super::DisplayServer::virtual_keyboard_show_ex]."]
#[must_use]
pub struct ExVirtualKeyboardShow < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, existing_text: CowArg < 'a, GString >, position: Rect2, type_: crate::classes::display_server::VirtualKeyboardType, max_length: i32, cursor_start: i32, cursor_end: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVirtualKeyboardShow < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, existing_text: impl AsArg < GString > + 'a,) -> Self {
        let position = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let type_ = crate::obj::EngineEnum::from_ord(0);
        let max_length = - 1i32;
        let cursor_start = - 1i32;
        let cursor_end = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, existing_text: existing_text.into_arg(), position: position, type_: type_, max_length: max_length, cursor_start: cursor_start, cursor_end: cursor_end,
        }
    }
    #[inline]
    pub fn position(self, position: Rect2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn type_(self, type_: crate::classes::display_server::VirtualKeyboardType) -> Self {
        Self {
            type_: type_, .. self
        }
    }
    #[inline]
    pub fn max_length(self, max_length: i32) -> Self {
        Self {
            max_length: max_length, .. self
        }
    }
    #[inline]
    pub fn cursor_start(self, cursor_start: i32) -> Self {
        Self {
            cursor_start: cursor_start, .. self
        }
    }
    #[inline]
    pub fn cursor_end(self, cursor_end: i32) -> Self {
        Self {
            cursor_end: cursor_end, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, existing_text, position, type_, max_length, cursor_start, cursor_end,
        }
        = self;
        re_export::DisplayServer::virtual_keyboard_show_full(surround_object, existing_text, position, type_, max_length, cursor_start, cursor_end,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::cursor_set_custom_image_ex`][super::DisplayServer::cursor_set_custom_image_ex]."]
#[must_use]
pub struct ExCursorSetCustomImage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, cursor: CowArg < 'a, Option < Gd < crate::classes::Resource > > >, shape: crate::classes::display_server::CursorShape, hotspot: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCursorSetCustomImage < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, cursor: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'a,) -> Self {
        let shape = crate::obj::EngineEnum::from_ord(0);
        let hotspot = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, cursor: cursor.into_arg(), shape: shape, hotspot: hotspot,
        }
    }
    #[inline]
    pub fn shape(self, shape: crate::classes::display_server::CursorShape) -> Self {
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
            _phantom, surround_object, cursor, shape, hotspot,
        }
        = self;
        re_export::DisplayServer::cursor_set_custom_image_full(surround_object, cursor, shape, hotspot,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::file_dialog_show_ex`][super::DisplayServer::file_dialog_show_ex]."]
#[must_use]
pub struct ExFileDialogShow < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, title: CowArg < 'a, GString >, current_directory: CowArg < 'a, GString >, filename: CowArg < 'a, GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: CowArg < 'a, PackedStringArray >, callback: CowArg < 'a, Callable >, parent_window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFileDialogShow < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, title: impl AsArg < GString > + 'a, current_directory: impl AsArg < GString > + 'a, filename: impl AsArg < GString > + 'a, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &'a PackedStringArray, callback: &'a Callable,) -> Self {
        let parent_window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, title: title.into_arg(), current_directory: current_directory.into_arg(), filename: filename.into_arg(), show_hidden: show_hidden, mode: mode, filters: CowArg::Borrowed(filters), callback: CowArg::Borrowed(callback), parent_window_id: parent_window_id,
        }
    }
    #[inline]
    pub fn parent_window_id(self, parent_window_id: i32) -> Self {
        Self {
            parent_window_id: parent_window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, title, current_directory, filename, show_hidden, mode, filters, callback, parent_window_id,
        }
        = self;
        re_export::DisplayServer::file_dialog_show_full(surround_object, title, current_directory, filename, show_hidden, mode, filters.cow_as_arg(), callback.cow_as_arg(), parent_window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::file_dialog_with_options_show_ex`][super::DisplayServer::file_dialog_with_options_show_ex]."]
#[must_use]
pub struct ExFileDialogWithOptionsShow < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, title: CowArg < 'a, GString >, current_directory: CowArg < 'a, GString >, root: CowArg < 'a, GString >, filename: CowArg < 'a, GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: CowArg < 'a, PackedStringArray >, options: CowArg < 'a, Array < VarDictionary > >, callback: CowArg < 'a, Callable >, parent_window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFileDialogWithOptionsShow < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, title: impl AsArg < GString > + 'a, current_directory: impl AsArg < GString > + 'a, root: impl AsArg < GString > + 'a, filename: impl AsArg < GString > + 'a, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &'a PackedStringArray, options: &'a Array < VarDictionary >, callback: &'a Callable,) -> Self {
        let parent_window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, title: title.into_arg(), current_directory: current_directory.into_arg(), root: root.into_arg(), filename: filename.into_arg(), show_hidden: show_hidden, mode: mode, filters: CowArg::Borrowed(filters), options: CowArg::Borrowed(options), callback: CowArg::Borrowed(callback), parent_window_id: parent_window_id,
        }
    }
    #[inline]
    pub fn parent_window_id(self, parent_window_id: i32) -> Self {
        Self {
            parent_window_id: parent_window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, title, current_directory, root, filename, show_hidden, mode, filters, options, callback, parent_window_id,
        }
        = self;
        re_export::DisplayServer::file_dialog_with_options_show_full(surround_object, title, current_directory, root, filename, show_hidden, mode, filters.cow_as_arg(), options.cow_as_arg(), callback.cow_as_arg(), parent_window_id,)
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
    #[doc(alias = "FEATURE_SUBWINDOWS")]
    #[doc = "Godot enumerator name: `FEATURE_SUBWINDOWS`"]
    pub const SUBWINDOWS: Feature = Feature {
        ord: 1i32
    };
    #[doc(alias = "FEATURE_TOUCHSCREEN")]
    #[doc = "Godot enumerator name: `FEATURE_TOUCHSCREEN`"]
    pub const TOUCHSCREEN: Feature = Feature {
        ord: 2i32
    };
    #[doc(alias = "FEATURE_MOUSE")]
    #[doc = "Godot enumerator name: `FEATURE_MOUSE`"]
    pub const MOUSE: Feature = Feature {
        ord: 3i32
    };
    #[doc(alias = "FEATURE_MOUSE_WARP")]
    #[doc = "Godot enumerator name: `FEATURE_MOUSE_WARP`"]
    pub const MOUSE_WARP: Feature = Feature {
        ord: 4i32
    };
    #[doc(alias = "FEATURE_CLIPBOARD")]
    #[doc = "Godot enumerator name: `FEATURE_CLIPBOARD`"]
    pub const CLIPBOARD: Feature = Feature {
        ord: 5i32
    };
    #[doc(alias = "FEATURE_VIRTUAL_KEYBOARD")]
    #[doc = "Godot enumerator name: `FEATURE_VIRTUAL_KEYBOARD`"]
    pub const VIRTUAL_KEYBOARD: Feature = Feature {
        ord: 6i32
    };
    #[doc(alias = "FEATURE_CURSOR_SHAPE")]
    #[doc = "Godot enumerator name: `FEATURE_CURSOR_SHAPE`"]
    pub const CURSOR_SHAPE: Feature = Feature {
        ord: 7i32
    };
    #[doc(alias = "FEATURE_CUSTOM_CURSOR_SHAPE")]
    #[doc = "Godot enumerator name: `FEATURE_CUSTOM_CURSOR_SHAPE`"]
    pub const CUSTOM_CURSOR_SHAPE: Feature = Feature {
        ord: 8i32
    };
    #[doc(alias = "FEATURE_NATIVE_DIALOG")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_DIALOG`"]
    pub const NATIVE_DIALOG: Feature = Feature {
        ord: 9i32
    };
    #[doc(alias = "FEATURE_IME")]
    #[doc = "Godot enumerator name: `FEATURE_IME`"]
    pub const IME: Feature = Feature {
        ord: 10i32
    };
    #[doc(alias = "FEATURE_WINDOW_TRANSPARENCY")]
    #[doc = "Godot enumerator name: `FEATURE_WINDOW_TRANSPARENCY`"]
    pub const WINDOW_TRANSPARENCY: Feature = Feature {
        ord: 11i32
    };
    #[doc(alias = "FEATURE_HIDPI")]
    #[doc = "Godot enumerator name: `FEATURE_HIDPI`"]
    pub const HIDPI: Feature = Feature {
        ord: 12i32
    };
    #[doc(alias = "FEATURE_ICON")]
    #[doc = "Godot enumerator name: `FEATURE_ICON`"]
    pub const ICON: Feature = Feature {
        ord: 13i32
    };
    #[doc(alias = "FEATURE_NATIVE_ICON")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_ICON`"]
    pub const NATIVE_ICON: Feature = Feature {
        ord: 14i32
    };
    #[doc(alias = "FEATURE_ORIENTATION")]
    #[doc = "Godot enumerator name: `FEATURE_ORIENTATION`"]
    pub const ORIENTATION: Feature = Feature {
        ord: 15i32
    };
    #[doc(alias = "FEATURE_SWAP_BUFFERS")]
    #[doc = "Godot enumerator name: `FEATURE_SWAP_BUFFERS`"]
    pub const SWAP_BUFFERS: Feature = Feature {
        ord: 16i32
    };
    #[doc(alias = "FEATURE_CLIPBOARD_PRIMARY")]
    #[doc = "Godot enumerator name: `FEATURE_CLIPBOARD_PRIMARY`"]
    pub const CLIPBOARD_PRIMARY: Feature = Feature {
        ord: 18i32
    };
    #[doc(alias = "FEATURE_TEXT_TO_SPEECH")]
    #[doc = "Godot enumerator name: `FEATURE_TEXT_TO_SPEECH`"]
    pub const TEXT_TO_SPEECH: Feature = Feature {
        ord: 19i32
    };
    #[doc(alias = "FEATURE_EXTEND_TO_TITLE")]
    #[doc = "Godot enumerator name: `FEATURE_EXTEND_TO_TITLE`"]
    pub const EXTEND_TO_TITLE: Feature = Feature {
        ord: 20i32
    };
    #[doc(alias = "FEATURE_SCREEN_CAPTURE")]
    #[doc = "Godot enumerator name: `FEATURE_SCREEN_CAPTURE`"]
    pub const SCREEN_CAPTURE: Feature = Feature {
        ord: 21i32
    };
    #[doc(alias = "FEATURE_STATUS_INDICATOR")]
    #[doc = "Godot enumerator name: `FEATURE_STATUS_INDICATOR`"]
    pub const STATUS_INDICATOR: Feature = Feature {
        ord: 22i32
    };
    #[doc(alias = "FEATURE_NATIVE_HELP")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_HELP`"]
    pub const NATIVE_HELP: Feature = Feature {
        ord: 23i32
    };
    #[doc(alias = "FEATURE_NATIVE_DIALOG_INPUT")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_DIALOG_INPUT`"]
    pub const NATIVE_DIALOG_INPUT: Feature = Feature {
        ord: 24i32
    };
    #[doc(alias = "FEATURE_NATIVE_DIALOG_FILE")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_DIALOG_FILE`"]
    pub const NATIVE_DIALOG_FILE: Feature = Feature {
        ord: 25i32
    };
    #[doc(alias = "FEATURE_NATIVE_DIALOG_FILE_EXTRA")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_DIALOG_FILE_EXTRA`"]
    pub const NATIVE_DIALOG_FILE_EXTRA: Feature = Feature {
        ord: 26i32
    };
    #[doc(alias = "FEATURE_WINDOW_DRAG")]
    #[doc = "Godot enumerator name: `FEATURE_WINDOW_DRAG`"]
    pub const WINDOW_DRAG: Feature = Feature {
        ord: 27i32
    };
    #[doc(alias = "FEATURE_SCREEN_EXCLUDE_FROM_CAPTURE")]
    #[doc = "Godot enumerator name: `FEATURE_SCREEN_EXCLUDE_FROM_CAPTURE`"]
    pub const SCREEN_EXCLUDE_FROM_CAPTURE: Feature = Feature {
        ord: 28i32
    };
    #[doc(alias = "FEATURE_WINDOW_EMBEDDING")]
    #[doc = "Godot enumerator name: `FEATURE_WINDOW_EMBEDDING`"]
    pub const WINDOW_EMBEDDING: Feature = Feature {
        ord: 29i32
    };
    #[doc(alias = "FEATURE_NATIVE_DIALOG_FILE_MIME")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_DIALOG_FILE_MIME`"]
    pub const NATIVE_DIALOG_FILE_MIME: Feature = Feature {
        ord: 30i32
    };
    #[doc(alias = "FEATURE_EMOJI_AND_SYMBOL_PICKER")]
    #[doc = "Godot enumerator name: `FEATURE_EMOJI_AND_SYMBOL_PICKER`"]
    pub const EMOJI_AND_SYMBOL_PICKER: Feature = Feature {
        ord: 31i32
    };
    #[doc(alias = "FEATURE_NATIVE_COLOR_PICKER")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_COLOR_PICKER`"]
    pub const NATIVE_COLOR_PICKER: Feature = Feature {
        ord: 32i32
    };
    #[doc(alias = "FEATURE_SELF_FITTING_WINDOWS")]
    #[doc = "Godot enumerator name: `FEATURE_SELF_FITTING_WINDOWS`"]
    pub const SELF_FITTING_WINDOWS: Feature = Feature {
        ord: 33i32
    };
    #[doc(alias = "FEATURE_ACCESSIBILITY_SCREEN_READER")]
    #[doc = "Godot enumerator name: `FEATURE_ACCESSIBILITY_SCREEN_READER`"]
    pub const ACCESSIBILITY_SCREEN_READER: Feature = Feature {
        ord: 34i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 => Some(Self {
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
            Self::GLOBAL_MENU => "GLOBAL_MENU", Self::SUBWINDOWS => "SUBWINDOWS", Self::TOUCHSCREEN => "TOUCHSCREEN", Self::MOUSE => "MOUSE", Self::MOUSE_WARP => "MOUSE_WARP", Self::CLIPBOARD => "CLIPBOARD", Self::VIRTUAL_KEYBOARD => "VIRTUAL_KEYBOARD", Self::CURSOR_SHAPE => "CURSOR_SHAPE", Self::CUSTOM_CURSOR_SHAPE => "CUSTOM_CURSOR_SHAPE", Self::NATIVE_DIALOG => "NATIVE_DIALOG", Self::IME => "IME", Self::WINDOW_TRANSPARENCY => "WINDOW_TRANSPARENCY", Self::HIDPI => "HIDPI", Self::ICON => "ICON", Self::NATIVE_ICON => "NATIVE_ICON", Self::ORIENTATION => "ORIENTATION", Self::SWAP_BUFFERS => "SWAP_BUFFERS", Self::CLIPBOARD_PRIMARY => "CLIPBOARD_PRIMARY", Self::TEXT_TO_SPEECH => "TEXT_TO_SPEECH", Self::EXTEND_TO_TITLE => "EXTEND_TO_TITLE", Self::SCREEN_CAPTURE => "SCREEN_CAPTURE", Self::STATUS_INDICATOR => "STATUS_INDICATOR", Self::NATIVE_HELP => "NATIVE_HELP", Self::NATIVE_DIALOG_INPUT => "NATIVE_DIALOG_INPUT", Self::NATIVE_DIALOG_FILE => "NATIVE_DIALOG_FILE", Self::NATIVE_DIALOG_FILE_EXTRA => "NATIVE_DIALOG_FILE_EXTRA", Self::WINDOW_DRAG => "WINDOW_DRAG", Self::SCREEN_EXCLUDE_FROM_CAPTURE => "SCREEN_EXCLUDE_FROM_CAPTURE", Self::WINDOW_EMBEDDING => "WINDOW_EMBEDDING", Self::NATIVE_DIALOG_FILE_MIME => "NATIVE_DIALOG_FILE_MIME", Self::EMOJI_AND_SYMBOL_PICKER => "EMOJI_AND_SYMBOL_PICKER", Self::NATIVE_COLOR_PICKER => "NATIVE_COLOR_PICKER", Self::SELF_FITTING_WINDOWS => "SELF_FITTING_WINDOWS", Self::ACCESSIBILITY_SCREEN_READER => "ACCESSIBILITY_SCREEN_READER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Feature::GLOBAL_MENU, Feature::SUBWINDOWS, Feature::TOUCHSCREEN, Feature::MOUSE, Feature::MOUSE_WARP, Feature::CLIPBOARD, Feature::VIRTUAL_KEYBOARD, Feature::CURSOR_SHAPE, Feature::CUSTOM_CURSOR_SHAPE, Feature::NATIVE_DIALOG, Feature::IME, Feature::WINDOW_TRANSPARENCY, Feature::HIDPI, Feature::ICON, Feature::NATIVE_ICON, Feature::ORIENTATION, Feature::SWAP_BUFFERS, Feature::CLIPBOARD_PRIMARY, Feature::TEXT_TO_SPEECH, Feature::EXTEND_TO_TITLE, Feature::SCREEN_CAPTURE, Feature::STATUS_INDICATOR, Feature::NATIVE_HELP, Feature::NATIVE_DIALOG_INPUT, Feature::NATIVE_DIALOG_FILE, Feature::NATIVE_DIALOG_FILE_EXTRA, Feature::WINDOW_DRAG, Feature::SCREEN_EXCLUDE_FROM_CAPTURE, Feature::WINDOW_EMBEDDING, Feature::NATIVE_DIALOG_FILE_MIME, Feature::EMOJI_AND_SYMBOL_PICKER, Feature::NATIVE_COLOR_PICKER, Feature::SELF_FITTING_WINDOWS, Feature::ACCESSIBILITY_SCREEN_READER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Feature >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GLOBAL_MENU", "FEATURE_GLOBAL_MENU", Feature::GLOBAL_MENU), crate::meta::inspect::EnumConstant::new("SUBWINDOWS", "FEATURE_SUBWINDOWS", Feature::SUBWINDOWS), crate::meta::inspect::EnumConstant::new("TOUCHSCREEN", "FEATURE_TOUCHSCREEN", Feature::TOUCHSCREEN), crate::meta::inspect::EnumConstant::new("MOUSE", "FEATURE_MOUSE", Feature::MOUSE), crate::meta::inspect::EnumConstant::new("MOUSE_WARP", "FEATURE_MOUSE_WARP", Feature::MOUSE_WARP), crate::meta::inspect::EnumConstant::new("CLIPBOARD", "FEATURE_CLIPBOARD", Feature::CLIPBOARD), crate::meta::inspect::EnumConstant::new("VIRTUAL_KEYBOARD", "FEATURE_VIRTUAL_KEYBOARD", Feature::VIRTUAL_KEYBOARD), crate::meta::inspect::EnumConstant::new("CURSOR_SHAPE", "FEATURE_CURSOR_SHAPE", Feature::CURSOR_SHAPE), crate::meta::inspect::EnumConstant::new("CUSTOM_CURSOR_SHAPE", "FEATURE_CUSTOM_CURSOR_SHAPE", Feature::CUSTOM_CURSOR_SHAPE), crate::meta::inspect::EnumConstant::new("NATIVE_DIALOG", "FEATURE_NATIVE_DIALOG", Feature::NATIVE_DIALOG), crate::meta::inspect::EnumConstant::new("IME", "FEATURE_IME", Feature::IME), crate::meta::inspect::EnumConstant::new("WINDOW_TRANSPARENCY", "FEATURE_WINDOW_TRANSPARENCY", Feature::WINDOW_TRANSPARENCY), crate::meta::inspect::EnumConstant::new("HIDPI", "FEATURE_HIDPI", Feature::HIDPI), crate::meta::inspect::EnumConstant::new("ICON", "FEATURE_ICON", Feature::ICON), crate::meta::inspect::EnumConstant::new("NATIVE_ICON", "FEATURE_NATIVE_ICON", Feature::NATIVE_ICON), crate::meta::inspect::EnumConstant::new("ORIENTATION", "FEATURE_ORIENTATION", Feature::ORIENTATION), crate::meta::inspect::EnumConstant::new("SWAP_BUFFERS", "FEATURE_SWAP_BUFFERS", Feature::SWAP_BUFFERS), crate::meta::inspect::EnumConstant::new("CLIPBOARD_PRIMARY", "FEATURE_CLIPBOARD_PRIMARY", Feature::CLIPBOARD_PRIMARY), crate::meta::inspect::EnumConstant::new("TEXT_TO_SPEECH", "FEATURE_TEXT_TO_SPEECH", Feature::TEXT_TO_SPEECH), crate::meta::inspect::EnumConstant::new("EXTEND_TO_TITLE", "FEATURE_EXTEND_TO_TITLE", Feature::EXTEND_TO_TITLE), crate::meta::inspect::EnumConstant::new("SCREEN_CAPTURE", "FEATURE_SCREEN_CAPTURE", Feature::SCREEN_CAPTURE), crate::meta::inspect::EnumConstant::new("STATUS_INDICATOR", "FEATURE_STATUS_INDICATOR", Feature::STATUS_INDICATOR), crate::meta::inspect::EnumConstant::new("NATIVE_HELP", "FEATURE_NATIVE_HELP", Feature::NATIVE_HELP), crate::meta::inspect::EnumConstant::new("NATIVE_DIALOG_INPUT", "FEATURE_NATIVE_DIALOG_INPUT", Feature::NATIVE_DIALOG_INPUT), crate::meta::inspect::EnumConstant::new("NATIVE_DIALOG_FILE", "FEATURE_NATIVE_DIALOG_FILE", Feature::NATIVE_DIALOG_FILE), crate::meta::inspect::EnumConstant::new("NATIVE_DIALOG_FILE_EXTRA", "FEATURE_NATIVE_DIALOG_FILE_EXTRA", Feature::NATIVE_DIALOG_FILE_EXTRA), crate::meta::inspect::EnumConstant::new("WINDOW_DRAG", "FEATURE_WINDOW_DRAG", Feature::WINDOW_DRAG), crate::meta::inspect::EnumConstant::new("SCREEN_EXCLUDE_FROM_CAPTURE", "FEATURE_SCREEN_EXCLUDE_FROM_CAPTURE", Feature::SCREEN_EXCLUDE_FROM_CAPTURE), crate::meta::inspect::EnumConstant::new("WINDOW_EMBEDDING", "FEATURE_WINDOW_EMBEDDING", Feature::WINDOW_EMBEDDING), crate::meta::inspect::EnumConstant::new("NATIVE_DIALOG_FILE_MIME", "FEATURE_NATIVE_DIALOG_FILE_MIME", Feature::NATIVE_DIALOG_FILE_MIME), crate::meta::inspect::EnumConstant::new("EMOJI_AND_SYMBOL_PICKER", "FEATURE_EMOJI_AND_SYMBOL_PICKER", Feature::EMOJI_AND_SYMBOL_PICKER), crate::meta::inspect::EnumConstant::new("NATIVE_COLOR_PICKER", "FEATURE_NATIVE_COLOR_PICKER", Feature::NATIVE_COLOR_PICKER), crate::meta::inspect::EnumConstant::new("SELF_FITTING_WINDOWS", "FEATURE_SELF_FITTING_WINDOWS", Feature::SELF_FITTING_WINDOWS), crate::meta::inspect::EnumConstant::new("ACCESSIBILITY_SCREEN_READER", "FEATURE_ACCESSIBILITY_SCREEN_READER", Feature::ACCESSIBILITY_SCREEN_READER)]
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
pub struct AccessibilityRole {
    ord: i32
}
impl AccessibilityRole {
    #[doc(alias = "ROLE_UNKNOWN")]
    #[doc = "Godot enumerator name: `ROLE_UNKNOWN`"]
    pub const UNKNOWN: AccessibilityRole = AccessibilityRole {
        ord: 0i32
    };
    #[doc(alias = "ROLE_DEFAULT_BUTTON")]
    #[doc = "Godot enumerator name: `ROLE_DEFAULT_BUTTON`"]
    pub const DEFAULT_BUTTON: AccessibilityRole = AccessibilityRole {
        ord: 1i32
    };
    #[doc(alias = "ROLE_AUDIO")]
    #[doc = "Godot enumerator name: `ROLE_AUDIO`"]
    pub const AUDIO: AccessibilityRole = AccessibilityRole {
        ord: 2i32
    };
    #[doc(alias = "ROLE_VIDEO")]
    #[doc = "Godot enumerator name: `ROLE_VIDEO`"]
    pub const VIDEO: AccessibilityRole = AccessibilityRole {
        ord: 3i32
    };
    #[doc(alias = "ROLE_STATIC_TEXT")]
    #[doc = "Godot enumerator name: `ROLE_STATIC_TEXT`"]
    pub const STATIC_TEXT: AccessibilityRole = AccessibilityRole {
        ord: 4i32
    };
    #[doc(alias = "ROLE_CONTAINER")]
    #[doc = "Godot enumerator name: `ROLE_CONTAINER`"]
    pub const CONTAINER: AccessibilityRole = AccessibilityRole {
        ord: 5i32
    };
    #[doc(alias = "ROLE_PANEL")]
    #[doc = "Godot enumerator name: `ROLE_PANEL`"]
    pub const PANEL: AccessibilityRole = AccessibilityRole {
        ord: 6i32
    };
    #[doc(alias = "ROLE_BUTTON")]
    #[doc = "Godot enumerator name: `ROLE_BUTTON`"]
    pub const BUTTON: AccessibilityRole = AccessibilityRole {
        ord: 7i32
    };
    #[doc(alias = "ROLE_LINK")]
    #[doc = "Godot enumerator name: `ROLE_LINK`"]
    pub const LINK: AccessibilityRole = AccessibilityRole {
        ord: 8i32
    };
    #[doc(alias = "ROLE_CHECK_BOX")]
    #[doc = "Godot enumerator name: `ROLE_CHECK_BOX`"]
    pub const CHECK_BOX: AccessibilityRole = AccessibilityRole {
        ord: 9i32
    };
    #[doc(alias = "ROLE_RADIO_BUTTON")]
    #[doc = "Godot enumerator name: `ROLE_RADIO_BUTTON`"]
    pub const RADIO_BUTTON: AccessibilityRole = AccessibilityRole {
        ord: 10i32
    };
    #[doc(alias = "ROLE_CHECK_BUTTON")]
    #[doc = "Godot enumerator name: `ROLE_CHECK_BUTTON`"]
    pub const CHECK_BUTTON: AccessibilityRole = AccessibilityRole {
        ord: 11i32
    };
    #[doc(alias = "ROLE_SCROLL_BAR")]
    #[doc = "Godot enumerator name: `ROLE_SCROLL_BAR`"]
    pub const SCROLL_BAR: AccessibilityRole = AccessibilityRole {
        ord: 12i32
    };
    #[doc(alias = "ROLE_SCROLL_VIEW")]
    #[doc = "Godot enumerator name: `ROLE_SCROLL_VIEW`"]
    pub const SCROLL_VIEW: AccessibilityRole = AccessibilityRole {
        ord: 13i32
    };
    #[doc(alias = "ROLE_SPLITTER")]
    #[doc = "Godot enumerator name: `ROLE_SPLITTER`"]
    pub const SPLITTER: AccessibilityRole = AccessibilityRole {
        ord: 14i32
    };
    #[doc(alias = "ROLE_SLIDER")]
    #[doc = "Godot enumerator name: `ROLE_SLIDER`"]
    pub const SLIDER: AccessibilityRole = AccessibilityRole {
        ord: 15i32
    };
    #[doc(alias = "ROLE_SPIN_BUTTON")]
    #[doc = "Godot enumerator name: `ROLE_SPIN_BUTTON`"]
    pub const SPIN_BUTTON: AccessibilityRole = AccessibilityRole {
        ord: 16i32
    };
    #[doc(alias = "ROLE_PROGRESS_INDICATOR")]
    #[doc = "Godot enumerator name: `ROLE_PROGRESS_INDICATOR`"]
    pub const PROGRESS_INDICATOR: AccessibilityRole = AccessibilityRole {
        ord: 17i32
    };
    #[doc(alias = "ROLE_TEXT_FIELD")]
    #[doc = "Godot enumerator name: `ROLE_TEXT_FIELD`"]
    pub const TEXT_FIELD: AccessibilityRole = AccessibilityRole {
        ord: 18i32
    };
    #[doc(alias = "ROLE_MULTILINE_TEXT_FIELD")]
    #[doc = "Godot enumerator name: `ROLE_MULTILINE_TEXT_FIELD`"]
    pub const MULTILINE_TEXT_FIELD: AccessibilityRole = AccessibilityRole {
        ord: 19i32
    };
    #[doc(alias = "ROLE_COLOR_PICKER")]
    #[doc = "Godot enumerator name: `ROLE_COLOR_PICKER`"]
    pub const COLOR_PICKER: AccessibilityRole = AccessibilityRole {
        ord: 20i32
    };
    #[doc(alias = "ROLE_TABLE")]
    #[doc = "Godot enumerator name: `ROLE_TABLE`"]
    pub const TABLE: AccessibilityRole = AccessibilityRole {
        ord: 21i32
    };
    #[doc(alias = "ROLE_CELL")]
    #[doc = "Godot enumerator name: `ROLE_CELL`"]
    pub const CELL: AccessibilityRole = AccessibilityRole {
        ord: 22i32
    };
    #[doc(alias = "ROLE_ROW")]
    #[doc = "Godot enumerator name: `ROLE_ROW`"]
    pub const ROW: AccessibilityRole = AccessibilityRole {
        ord: 23i32
    };
    #[doc(alias = "ROLE_ROW_GROUP")]
    #[doc = "Godot enumerator name: `ROLE_ROW_GROUP`"]
    pub const ROW_GROUP: AccessibilityRole = AccessibilityRole {
        ord: 24i32
    };
    #[doc(alias = "ROLE_ROW_HEADER")]
    #[doc = "Godot enumerator name: `ROLE_ROW_HEADER`"]
    pub const ROW_HEADER: AccessibilityRole = AccessibilityRole {
        ord: 25i32
    };
    #[doc(alias = "ROLE_COLUMN_HEADER")]
    #[doc = "Godot enumerator name: `ROLE_COLUMN_HEADER`"]
    pub const COLUMN_HEADER: AccessibilityRole = AccessibilityRole {
        ord: 26i32
    };
    #[doc(alias = "ROLE_TREE")]
    #[doc = "Godot enumerator name: `ROLE_TREE`"]
    pub const TREE: AccessibilityRole = AccessibilityRole {
        ord: 27i32
    };
    #[doc(alias = "ROLE_TREE_ITEM")]
    #[doc = "Godot enumerator name: `ROLE_TREE_ITEM`"]
    pub const TREE_ITEM: AccessibilityRole = AccessibilityRole {
        ord: 28i32
    };
    #[doc(alias = "ROLE_LIST")]
    #[doc = "Godot enumerator name: `ROLE_LIST`"]
    pub const LIST: AccessibilityRole = AccessibilityRole {
        ord: 29i32
    };
    #[doc(alias = "ROLE_LIST_ITEM")]
    #[doc = "Godot enumerator name: `ROLE_LIST_ITEM`"]
    pub const LIST_ITEM: AccessibilityRole = AccessibilityRole {
        ord: 30i32
    };
    #[doc(alias = "ROLE_LIST_BOX")]
    #[doc = "Godot enumerator name: `ROLE_LIST_BOX`"]
    pub const LIST_BOX: AccessibilityRole = AccessibilityRole {
        ord: 31i32
    };
    #[doc(alias = "ROLE_LIST_BOX_OPTION")]
    #[doc = "Godot enumerator name: `ROLE_LIST_BOX_OPTION`"]
    pub const LIST_BOX_OPTION: AccessibilityRole = AccessibilityRole {
        ord: 32i32
    };
    #[doc(alias = "ROLE_TAB_BAR")]
    #[doc = "Godot enumerator name: `ROLE_TAB_BAR`"]
    pub const TAB_BAR: AccessibilityRole = AccessibilityRole {
        ord: 33i32
    };
    #[doc(alias = "ROLE_TAB")]
    #[doc = "Godot enumerator name: `ROLE_TAB`"]
    pub const TAB: AccessibilityRole = AccessibilityRole {
        ord: 34i32
    };
    #[doc(alias = "ROLE_TAB_PANEL")]
    #[doc = "Godot enumerator name: `ROLE_TAB_PANEL`"]
    pub const TAB_PANEL: AccessibilityRole = AccessibilityRole {
        ord: 35i32
    };
    #[doc(alias = "ROLE_MENU_BAR")]
    #[doc = "Godot enumerator name: `ROLE_MENU_BAR`"]
    pub const MENU_BAR: AccessibilityRole = AccessibilityRole {
        ord: 36i32
    };
    #[doc(alias = "ROLE_MENU")]
    #[doc = "Godot enumerator name: `ROLE_MENU`"]
    pub const MENU: AccessibilityRole = AccessibilityRole {
        ord: 37i32
    };
    #[doc(alias = "ROLE_MENU_ITEM")]
    #[doc = "Godot enumerator name: `ROLE_MENU_ITEM`"]
    pub const MENU_ITEM: AccessibilityRole = AccessibilityRole {
        ord: 38i32
    };
    #[doc(alias = "ROLE_MENU_ITEM_CHECK_BOX")]
    #[doc = "Godot enumerator name: `ROLE_MENU_ITEM_CHECK_BOX`"]
    pub const MENU_ITEM_CHECK_BOX: AccessibilityRole = AccessibilityRole {
        ord: 39i32
    };
    #[doc(alias = "ROLE_MENU_ITEM_RADIO")]
    #[doc = "Godot enumerator name: `ROLE_MENU_ITEM_RADIO`"]
    pub const MENU_ITEM_RADIO: AccessibilityRole = AccessibilityRole {
        ord: 40i32
    };
    #[doc(alias = "ROLE_IMAGE")]
    #[doc = "Godot enumerator name: `ROLE_IMAGE`"]
    pub const IMAGE: AccessibilityRole = AccessibilityRole {
        ord: 41i32
    };
    #[doc(alias = "ROLE_WINDOW")]
    #[doc = "Godot enumerator name: `ROLE_WINDOW`"]
    pub const WINDOW: AccessibilityRole = AccessibilityRole {
        ord: 42i32
    };
    #[doc(alias = "ROLE_TITLE_BAR")]
    #[doc = "Godot enumerator name: `ROLE_TITLE_BAR`"]
    pub const TITLE_BAR: AccessibilityRole = AccessibilityRole {
        ord: 43i32
    };
    #[doc(alias = "ROLE_DIALOG")]
    #[doc = "Godot enumerator name: `ROLE_DIALOG`"]
    pub const DIALOG: AccessibilityRole = AccessibilityRole {
        ord: 44i32
    };
    #[doc(alias = "ROLE_TOOLTIP")]
    #[doc = "Godot enumerator name: `ROLE_TOOLTIP`"]
    pub const TOOLTIP: AccessibilityRole = AccessibilityRole {
        ord: 45i32
    };
    
}
impl std::fmt::Debug for AccessibilityRole {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityRole") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityRole {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 | ord @ 40i32 | ord @ 41i32 | ord @ 42i32 | ord @ 43i32 | ord @ 44i32 | ord @ 45i32 => Some(Self {
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
            Self::UNKNOWN => "UNKNOWN", Self::DEFAULT_BUTTON => "DEFAULT_BUTTON", Self::AUDIO => "AUDIO", Self::VIDEO => "VIDEO", Self::STATIC_TEXT => "STATIC_TEXT", Self::CONTAINER => "CONTAINER", Self::PANEL => "PANEL", Self::BUTTON => "BUTTON", Self::LINK => "LINK", Self::CHECK_BOX => "CHECK_BOX", Self::RADIO_BUTTON => "RADIO_BUTTON", Self::CHECK_BUTTON => "CHECK_BUTTON", Self::SCROLL_BAR => "SCROLL_BAR", Self::SCROLL_VIEW => "SCROLL_VIEW", Self::SPLITTER => "SPLITTER", Self::SLIDER => "SLIDER", Self::SPIN_BUTTON => "SPIN_BUTTON", Self::PROGRESS_INDICATOR => "PROGRESS_INDICATOR", Self::TEXT_FIELD => "TEXT_FIELD", Self::MULTILINE_TEXT_FIELD => "MULTILINE_TEXT_FIELD", Self::COLOR_PICKER => "COLOR_PICKER", Self::TABLE => "TABLE", Self::CELL => "CELL", Self::ROW => "ROW", Self::ROW_GROUP => "ROW_GROUP", Self::ROW_HEADER => "ROW_HEADER", Self::COLUMN_HEADER => "COLUMN_HEADER", Self::TREE => "TREE", Self::TREE_ITEM => "TREE_ITEM", Self::LIST => "LIST", Self::LIST_ITEM => "LIST_ITEM", Self::LIST_BOX => "LIST_BOX", Self::LIST_BOX_OPTION => "LIST_BOX_OPTION", Self::TAB_BAR => "TAB_BAR", Self::TAB => "TAB", Self::TAB_PANEL => "TAB_PANEL", Self::MENU_BAR => "MENU_BAR", Self::MENU => "MENU", Self::MENU_ITEM => "MENU_ITEM", Self::MENU_ITEM_CHECK_BOX => "MENU_ITEM_CHECK_BOX", Self::MENU_ITEM_RADIO => "MENU_ITEM_RADIO", Self::IMAGE => "IMAGE", Self::WINDOW => "WINDOW", Self::TITLE_BAR => "TITLE_BAR", Self::DIALOG => "DIALOG", Self::TOOLTIP => "TOOLTIP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityRole::UNKNOWN, AccessibilityRole::DEFAULT_BUTTON, AccessibilityRole::AUDIO, AccessibilityRole::VIDEO, AccessibilityRole::STATIC_TEXT, AccessibilityRole::CONTAINER, AccessibilityRole::PANEL, AccessibilityRole::BUTTON, AccessibilityRole::LINK, AccessibilityRole::CHECK_BOX, AccessibilityRole::RADIO_BUTTON, AccessibilityRole::CHECK_BUTTON, AccessibilityRole::SCROLL_BAR, AccessibilityRole::SCROLL_VIEW, AccessibilityRole::SPLITTER, AccessibilityRole::SLIDER, AccessibilityRole::SPIN_BUTTON, AccessibilityRole::PROGRESS_INDICATOR, AccessibilityRole::TEXT_FIELD, AccessibilityRole::MULTILINE_TEXT_FIELD, AccessibilityRole::COLOR_PICKER, AccessibilityRole::TABLE, AccessibilityRole::CELL, AccessibilityRole::ROW, AccessibilityRole::ROW_GROUP, AccessibilityRole::ROW_HEADER, AccessibilityRole::COLUMN_HEADER, AccessibilityRole::TREE, AccessibilityRole::TREE_ITEM, AccessibilityRole::LIST, AccessibilityRole::LIST_ITEM, AccessibilityRole::LIST_BOX, AccessibilityRole::LIST_BOX_OPTION, AccessibilityRole::TAB_BAR, AccessibilityRole::TAB, AccessibilityRole::TAB_PANEL, AccessibilityRole::MENU_BAR, AccessibilityRole::MENU, AccessibilityRole::MENU_ITEM, AccessibilityRole::MENU_ITEM_CHECK_BOX, AccessibilityRole::MENU_ITEM_RADIO, AccessibilityRole::IMAGE, AccessibilityRole::WINDOW, AccessibilityRole::TITLE_BAR, AccessibilityRole::DIALOG, AccessibilityRole::TOOLTIP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityRole >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNKNOWN", "ROLE_UNKNOWN", AccessibilityRole::UNKNOWN), crate::meta::inspect::EnumConstant::new("DEFAULT_BUTTON", "ROLE_DEFAULT_BUTTON", AccessibilityRole::DEFAULT_BUTTON), crate::meta::inspect::EnumConstant::new("AUDIO", "ROLE_AUDIO", AccessibilityRole::AUDIO), crate::meta::inspect::EnumConstant::new("VIDEO", "ROLE_VIDEO", AccessibilityRole::VIDEO), crate::meta::inspect::EnumConstant::new("STATIC_TEXT", "ROLE_STATIC_TEXT", AccessibilityRole::STATIC_TEXT), crate::meta::inspect::EnumConstant::new("CONTAINER", "ROLE_CONTAINER", AccessibilityRole::CONTAINER), crate::meta::inspect::EnumConstant::new("PANEL", "ROLE_PANEL", AccessibilityRole::PANEL), crate::meta::inspect::EnumConstant::new("BUTTON", "ROLE_BUTTON", AccessibilityRole::BUTTON), crate::meta::inspect::EnumConstant::new("LINK", "ROLE_LINK", AccessibilityRole::LINK), crate::meta::inspect::EnumConstant::new("CHECK_BOX", "ROLE_CHECK_BOX", AccessibilityRole::CHECK_BOX), crate::meta::inspect::EnumConstant::new("RADIO_BUTTON", "ROLE_RADIO_BUTTON", AccessibilityRole::RADIO_BUTTON), crate::meta::inspect::EnumConstant::new("CHECK_BUTTON", "ROLE_CHECK_BUTTON", AccessibilityRole::CHECK_BUTTON), crate::meta::inspect::EnumConstant::new("SCROLL_BAR", "ROLE_SCROLL_BAR", AccessibilityRole::SCROLL_BAR), crate::meta::inspect::EnumConstant::new("SCROLL_VIEW", "ROLE_SCROLL_VIEW", AccessibilityRole::SCROLL_VIEW), crate::meta::inspect::EnumConstant::new("SPLITTER", "ROLE_SPLITTER", AccessibilityRole::SPLITTER), crate::meta::inspect::EnumConstant::new("SLIDER", "ROLE_SLIDER", AccessibilityRole::SLIDER), crate::meta::inspect::EnumConstant::new("SPIN_BUTTON", "ROLE_SPIN_BUTTON", AccessibilityRole::SPIN_BUTTON), crate::meta::inspect::EnumConstant::new("PROGRESS_INDICATOR", "ROLE_PROGRESS_INDICATOR", AccessibilityRole::PROGRESS_INDICATOR), crate::meta::inspect::EnumConstant::new("TEXT_FIELD", "ROLE_TEXT_FIELD", AccessibilityRole::TEXT_FIELD), crate::meta::inspect::EnumConstant::new("MULTILINE_TEXT_FIELD", "ROLE_MULTILINE_TEXT_FIELD", AccessibilityRole::MULTILINE_TEXT_FIELD), crate::meta::inspect::EnumConstant::new("COLOR_PICKER", "ROLE_COLOR_PICKER", AccessibilityRole::COLOR_PICKER), crate::meta::inspect::EnumConstant::new("TABLE", "ROLE_TABLE", AccessibilityRole::TABLE), crate::meta::inspect::EnumConstant::new("CELL", "ROLE_CELL", AccessibilityRole::CELL), crate::meta::inspect::EnumConstant::new("ROW", "ROLE_ROW", AccessibilityRole::ROW), crate::meta::inspect::EnumConstant::new("ROW_GROUP", "ROLE_ROW_GROUP", AccessibilityRole::ROW_GROUP), crate::meta::inspect::EnumConstant::new("ROW_HEADER", "ROLE_ROW_HEADER", AccessibilityRole::ROW_HEADER), crate::meta::inspect::EnumConstant::new("COLUMN_HEADER", "ROLE_COLUMN_HEADER", AccessibilityRole::COLUMN_HEADER), crate::meta::inspect::EnumConstant::new("TREE", "ROLE_TREE", AccessibilityRole::TREE), crate::meta::inspect::EnumConstant::new("TREE_ITEM", "ROLE_TREE_ITEM", AccessibilityRole::TREE_ITEM), crate::meta::inspect::EnumConstant::new("LIST", "ROLE_LIST", AccessibilityRole::LIST), crate::meta::inspect::EnumConstant::new("LIST_ITEM", "ROLE_LIST_ITEM", AccessibilityRole::LIST_ITEM), crate::meta::inspect::EnumConstant::new("LIST_BOX", "ROLE_LIST_BOX", AccessibilityRole::LIST_BOX), crate::meta::inspect::EnumConstant::new("LIST_BOX_OPTION", "ROLE_LIST_BOX_OPTION", AccessibilityRole::LIST_BOX_OPTION), crate::meta::inspect::EnumConstant::new("TAB_BAR", "ROLE_TAB_BAR", AccessibilityRole::TAB_BAR), crate::meta::inspect::EnumConstant::new("TAB", "ROLE_TAB", AccessibilityRole::TAB), crate::meta::inspect::EnumConstant::new("TAB_PANEL", "ROLE_TAB_PANEL", AccessibilityRole::TAB_PANEL), crate::meta::inspect::EnumConstant::new("MENU_BAR", "ROLE_MENU_BAR", AccessibilityRole::MENU_BAR), crate::meta::inspect::EnumConstant::new("MENU", "ROLE_MENU", AccessibilityRole::MENU), crate::meta::inspect::EnumConstant::new("MENU_ITEM", "ROLE_MENU_ITEM", AccessibilityRole::MENU_ITEM), crate::meta::inspect::EnumConstant::new("MENU_ITEM_CHECK_BOX", "ROLE_MENU_ITEM_CHECK_BOX", AccessibilityRole::MENU_ITEM_CHECK_BOX), crate::meta::inspect::EnumConstant::new("MENU_ITEM_RADIO", "ROLE_MENU_ITEM_RADIO", AccessibilityRole::MENU_ITEM_RADIO), crate::meta::inspect::EnumConstant::new("IMAGE", "ROLE_IMAGE", AccessibilityRole::IMAGE), crate::meta::inspect::EnumConstant::new("WINDOW", "ROLE_WINDOW", AccessibilityRole::WINDOW), crate::meta::inspect::EnumConstant::new("TITLE_BAR", "ROLE_TITLE_BAR", AccessibilityRole::TITLE_BAR), crate::meta::inspect::EnumConstant::new("DIALOG", "ROLE_DIALOG", AccessibilityRole::DIALOG), crate::meta::inspect::EnumConstant::new("TOOLTIP", "ROLE_TOOLTIP", AccessibilityRole::TOOLTIP)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityRole {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AccessibilityRole {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityRole {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AccessibilityPopupType {
    ord: i32
}
impl AccessibilityPopupType {
    #[doc(alias = "POPUP_MENU")]
    #[doc = "Godot enumerator name: `POPUP_MENU`"]
    pub const MENU: AccessibilityPopupType = AccessibilityPopupType {
        ord: 0i32
    };
    #[doc(alias = "POPUP_LIST")]
    #[doc = "Godot enumerator name: `POPUP_LIST`"]
    pub const LIST: AccessibilityPopupType = AccessibilityPopupType {
        ord: 1i32
    };
    #[doc(alias = "POPUP_TREE")]
    #[doc = "Godot enumerator name: `POPUP_TREE`"]
    pub const TREE: AccessibilityPopupType = AccessibilityPopupType {
        ord: 2i32
    };
    #[doc(alias = "POPUP_DIALOG")]
    #[doc = "Godot enumerator name: `POPUP_DIALOG`"]
    pub const DIALOG: AccessibilityPopupType = AccessibilityPopupType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for AccessibilityPopupType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityPopupType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityPopupType {
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
            Self::MENU => "MENU", Self::LIST => "LIST", Self::TREE => "TREE", Self::DIALOG => "DIALOG", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityPopupType::MENU, AccessibilityPopupType::LIST, AccessibilityPopupType::TREE, AccessibilityPopupType::DIALOG]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityPopupType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MENU", "POPUP_MENU", AccessibilityPopupType::MENU), crate::meta::inspect::EnumConstant::new("LIST", "POPUP_LIST", AccessibilityPopupType::LIST), crate::meta::inspect::EnumConstant::new("TREE", "POPUP_TREE", AccessibilityPopupType::TREE), crate::meta::inspect::EnumConstant::new("DIALOG", "POPUP_DIALOG", AccessibilityPopupType::DIALOG)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityPopupType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AccessibilityPopupType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityPopupType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AccessibilityFlags {
    ord: i32
}
impl AccessibilityFlags {
    #[doc(alias = "FLAG_HIDDEN")]
    #[doc = "Godot enumerator name: `FLAG_HIDDEN`"]
    pub const HIDDEN: AccessibilityFlags = AccessibilityFlags {
        ord: 0i32
    };
    #[doc(alias = "FLAG_MULTISELECTABLE")]
    #[doc = "Godot enumerator name: `FLAG_MULTISELECTABLE`"]
    pub const MULTISELECTABLE: AccessibilityFlags = AccessibilityFlags {
        ord: 1i32
    };
    #[doc(alias = "FLAG_REQUIRED")]
    #[doc = "Godot enumerator name: `FLAG_REQUIRED`"]
    pub const REQUIRED: AccessibilityFlags = AccessibilityFlags {
        ord: 2i32
    };
    #[doc(alias = "FLAG_VISITED")]
    #[doc = "Godot enumerator name: `FLAG_VISITED`"]
    pub const VISITED: AccessibilityFlags = AccessibilityFlags {
        ord: 3i32
    };
    #[doc(alias = "FLAG_BUSY")]
    #[doc = "Godot enumerator name: `FLAG_BUSY`"]
    pub const BUSY: AccessibilityFlags = AccessibilityFlags {
        ord: 4i32
    };
    #[doc(alias = "FLAG_MODAL")]
    #[doc = "Godot enumerator name: `FLAG_MODAL`"]
    pub const MODAL: AccessibilityFlags = AccessibilityFlags {
        ord: 5i32
    };
    #[doc(alias = "FLAG_TOUCH_PASSTHROUGH")]
    #[doc = "Godot enumerator name: `FLAG_TOUCH_PASSTHROUGH`"]
    pub const TOUCH_PASSTHROUGH: AccessibilityFlags = AccessibilityFlags {
        ord: 6i32
    };
    #[doc(alias = "FLAG_READONLY")]
    #[doc = "Godot enumerator name: `FLAG_READONLY`"]
    pub const READONLY: AccessibilityFlags = AccessibilityFlags {
        ord: 7i32
    };
    #[doc(alias = "FLAG_DISABLED")]
    #[doc = "Godot enumerator name: `FLAG_DISABLED`"]
    pub const DISABLED: AccessibilityFlags = AccessibilityFlags {
        ord: 8i32
    };
    #[doc(alias = "FLAG_CLIPS_CHILDREN")]
    #[doc = "Godot enumerator name: `FLAG_CLIPS_CHILDREN`"]
    pub const CLIPS_CHILDREN: AccessibilityFlags = AccessibilityFlags {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for AccessibilityFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
            Self::HIDDEN => "HIDDEN", Self::MULTISELECTABLE => "MULTISELECTABLE", Self::REQUIRED => "REQUIRED", Self::VISITED => "VISITED", Self::BUSY => "BUSY", Self::MODAL => "MODAL", Self::TOUCH_PASSTHROUGH => "TOUCH_PASSTHROUGH", Self::READONLY => "READONLY", Self::DISABLED => "DISABLED", Self::CLIPS_CHILDREN => "CLIPS_CHILDREN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityFlags::HIDDEN, AccessibilityFlags::MULTISELECTABLE, AccessibilityFlags::REQUIRED, AccessibilityFlags::VISITED, AccessibilityFlags::BUSY, AccessibilityFlags::MODAL, AccessibilityFlags::TOUCH_PASSTHROUGH, AccessibilityFlags::READONLY, AccessibilityFlags::DISABLED, AccessibilityFlags::CLIPS_CHILDREN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("HIDDEN", "FLAG_HIDDEN", AccessibilityFlags::HIDDEN), crate::meta::inspect::EnumConstant::new("MULTISELECTABLE", "FLAG_MULTISELECTABLE", AccessibilityFlags::MULTISELECTABLE), crate::meta::inspect::EnumConstant::new("REQUIRED", "FLAG_REQUIRED", AccessibilityFlags::REQUIRED), crate::meta::inspect::EnumConstant::new("VISITED", "FLAG_VISITED", AccessibilityFlags::VISITED), crate::meta::inspect::EnumConstant::new("BUSY", "FLAG_BUSY", AccessibilityFlags::BUSY), crate::meta::inspect::EnumConstant::new("MODAL", "FLAG_MODAL", AccessibilityFlags::MODAL), crate::meta::inspect::EnumConstant::new("TOUCH_PASSTHROUGH", "FLAG_TOUCH_PASSTHROUGH", AccessibilityFlags::TOUCH_PASSTHROUGH), crate::meta::inspect::EnumConstant::new("READONLY", "FLAG_READONLY", AccessibilityFlags::READONLY), crate::meta::inspect::EnumConstant::new("DISABLED", "FLAG_DISABLED", AccessibilityFlags::DISABLED), crate::meta::inspect::EnumConstant::new("CLIPS_CHILDREN", "FLAG_CLIPS_CHILDREN", AccessibilityFlags::CLIPS_CHILDREN)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AccessibilityFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AccessibilityAction {
    ord: i32
}
impl AccessibilityAction {
    #[doc(alias = "ACTION_CLICK")]
    #[doc = "Godot enumerator name: `ACTION_CLICK`"]
    pub const CLICK: AccessibilityAction = AccessibilityAction {
        ord: 0i32
    };
    #[doc(alias = "ACTION_FOCUS")]
    #[doc = "Godot enumerator name: `ACTION_FOCUS`"]
    pub const FOCUS: AccessibilityAction = AccessibilityAction {
        ord: 1i32
    };
    #[doc(alias = "ACTION_BLUR")]
    #[doc = "Godot enumerator name: `ACTION_BLUR`"]
    pub const BLUR: AccessibilityAction = AccessibilityAction {
        ord: 2i32
    };
    #[doc(alias = "ACTION_COLLAPSE")]
    #[doc = "Godot enumerator name: `ACTION_COLLAPSE`"]
    pub const COLLAPSE: AccessibilityAction = AccessibilityAction {
        ord: 3i32
    };
    #[doc(alias = "ACTION_EXPAND")]
    #[doc = "Godot enumerator name: `ACTION_EXPAND`"]
    pub const EXPAND: AccessibilityAction = AccessibilityAction {
        ord: 4i32
    };
    #[doc(alias = "ACTION_DECREMENT")]
    #[doc = "Godot enumerator name: `ACTION_DECREMENT`"]
    pub const DECREMENT: AccessibilityAction = AccessibilityAction {
        ord: 5i32
    };
    #[doc(alias = "ACTION_INCREMENT")]
    #[doc = "Godot enumerator name: `ACTION_INCREMENT`"]
    pub const INCREMENT: AccessibilityAction = AccessibilityAction {
        ord: 6i32
    };
    #[doc(alias = "ACTION_HIDE_TOOLTIP")]
    #[doc = "Godot enumerator name: `ACTION_HIDE_TOOLTIP`"]
    pub const HIDE_TOOLTIP: AccessibilityAction = AccessibilityAction {
        ord: 7i32
    };
    #[doc(alias = "ACTION_SHOW_TOOLTIP")]
    #[doc = "Godot enumerator name: `ACTION_SHOW_TOOLTIP`"]
    pub const SHOW_TOOLTIP: AccessibilityAction = AccessibilityAction {
        ord: 8i32
    };
    #[doc(alias = "ACTION_SET_TEXT_SELECTION")]
    #[doc = "Godot enumerator name: `ACTION_SET_TEXT_SELECTION`"]
    pub const SET_TEXT_SELECTION: AccessibilityAction = AccessibilityAction {
        ord: 9i32
    };
    #[doc(alias = "ACTION_REPLACE_SELECTED_TEXT")]
    #[doc = "Godot enumerator name: `ACTION_REPLACE_SELECTED_TEXT`"]
    pub const REPLACE_SELECTED_TEXT: AccessibilityAction = AccessibilityAction {
        ord: 10i32
    };
    #[doc(alias = "ACTION_SCROLL_BACKWARD")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_BACKWARD`"]
    pub const SCROLL_BACKWARD: AccessibilityAction = AccessibilityAction {
        ord: 11i32
    };
    #[doc(alias = "ACTION_SCROLL_DOWN")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_DOWN`"]
    pub const SCROLL_DOWN: AccessibilityAction = AccessibilityAction {
        ord: 12i32
    };
    #[doc(alias = "ACTION_SCROLL_FORWARD")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_FORWARD`"]
    pub const SCROLL_FORWARD: AccessibilityAction = AccessibilityAction {
        ord: 13i32
    };
    #[doc(alias = "ACTION_SCROLL_LEFT")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_LEFT`"]
    pub const SCROLL_LEFT: AccessibilityAction = AccessibilityAction {
        ord: 14i32
    };
    #[doc(alias = "ACTION_SCROLL_RIGHT")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_RIGHT`"]
    pub const SCROLL_RIGHT: AccessibilityAction = AccessibilityAction {
        ord: 15i32
    };
    #[doc(alias = "ACTION_SCROLL_UP")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_UP`"]
    pub const SCROLL_UP: AccessibilityAction = AccessibilityAction {
        ord: 16i32
    };
    #[doc(alias = "ACTION_SCROLL_INTO_VIEW")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_INTO_VIEW`"]
    pub const SCROLL_INTO_VIEW: AccessibilityAction = AccessibilityAction {
        ord: 17i32
    };
    #[doc(alias = "ACTION_SCROLL_TO_POINT")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_TO_POINT`"]
    pub const SCROLL_TO_POINT: AccessibilityAction = AccessibilityAction {
        ord: 18i32
    };
    #[doc(alias = "ACTION_SET_SCROLL_OFFSET")]
    #[doc = "Godot enumerator name: `ACTION_SET_SCROLL_OFFSET`"]
    pub const SET_SCROLL_OFFSET: AccessibilityAction = AccessibilityAction {
        ord: 19i32
    };
    #[doc(alias = "ACTION_SET_VALUE")]
    #[doc = "Godot enumerator name: `ACTION_SET_VALUE`"]
    pub const SET_VALUE: AccessibilityAction = AccessibilityAction {
        ord: 20i32
    };
    #[doc(alias = "ACTION_SHOW_CONTEXT_MENU")]
    #[doc = "Godot enumerator name: `ACTION_SHOW_CONTEXT_MENU`"]
    pub const SHOW_CONTEXT_MENU: AccessibilityAction = AccessibilityAction {
        ord: 21i32
    };
    #[doc(alias = "ACTION_CUSTOM")]
    #[doc = "Godot enumerator name: `ACTION_CUSTOM`"]
    pub const CUSTOM: AccessibilityAction = AccessibilityAction {
        ord: 22i32
    };
    
}
impl std::fmt::Debug for AccessibilityAction {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityAction") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityAction {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 => Some(Self {
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
            Self::CLICK => "CLICK", Self::FOCUS => "FOCUS", Self::BLUR => "BLUR", Self::COLLAPSE => "COLLAPSE", Self::EXPAND => "EXPAND", Self::DECREMENT => "DECREMENT", Self::INCREMENT => "INCREMENT", Self::HIDE_TOOLTIP => "HIDE_TOOLTIP", Self::SHOW_TOOLTIP => "SHOW_TOOLTIP", Self::SET_TEXT_SELECTION => "SET_TEXT_SELECTION", Self::REPLACE_SELECTED_TEXT => "REPLACE_SELECTED_TEXT", Self::SCROLL_BACKWARD => "SCROLL_BACKWARD", Self::SCROLL_DOWN => "SCROLL_DOWN", Self::SCROLL_FORWARD => "SCROLL_FORWARD", Self::SCROLL_LEFT => "SCROLL_LEFT", Self::SCROLL_RIGHT => "SCROLL_RIGHT", Self::SCROLL_UP => "SCROLL_UP", Self::SCROLL_INTO_VIEW => "SCROLL_INTO_VIEW", Self::SCROLL_TO_POINT => "SCROLL_TO_POINT", Self::SET_SCROLL_OFFSET => "SET_SCROLL_OFFSET", Self::SET_VALUE => "SET_VALUE", Self::SHOW_CONTEXT_MENU => "SHOW_CONTEXT_MENU", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityAction::CLICK, AccessibilityAction::FOCUS, AccessibilityAction::BLUR, AccessibilityAction::COLLAPSE, AccessibilityAction::EXPAND, AccessibilityAction::DECREMENT, AccessibilityAction::INCREMENT, AccessibilityAction::HIDE_TOOLTIP, AccessibilityAction::SHOW_TOOLTIP, AccessibilityAction::SET_TEXT_SELECTION, AccessibilityAction::REPLACE_SELECTED_TEXT, AccessibilityAction::SCROLL_BACKWARD, AccessibilityAction::SCROLL_DOWN, AccessibilityAction::SCROLL_FORWARD, AccessibilityAction::SCROLL_LEFT, AccessibilityAction::SCROLL_RIGHT, AccessibilityAction::SCROLL_UP, AccessibilityAction::SCROLL_INTO_VIEW, AccessibilityAction::SCROLL_TO_POINT, AccessibilityAction::SET_SCROLL_OFFSET, AccessibilityAction::SET_VALUE, AccessibilityAction::SHOW_CONTEXT_MENU, AccessibilityAction::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityAction >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CLICK", "ACTION_CLICK", AccessibilityAction::CLICK), crate::meta::inspect::EnumConstant::new("FOCUS", "ACTION_FOCUS", AccessibilityAction::FOCUS), crate::meta::inspect::EnumConstant::new("BLUR", "ACTION_BLUR", AccessibilityAction::BLUR), crate::meta::inspect::EnumConstant::new("COLLAPSE", "ACTION_COLLAPSE", AccessibilityAction::COLLAPSE), crate::meta::inspect::EnumConstant::new("EXPAND", "ACTION_EXPAND", AccessibilityAction::EXPAND), crate::meta::inspect::EnumConstant::new("DECREMENT", "ACTION_DECREMENT", AccessibilityAction::DECREMENT), crate::meta::inspect::EnumConstant::new("INCREMENT", "ACTION_INCREMENT", AccessibilityAction::INCREMENT), crate::meta::inspect::EnumConstant::new("HIDE_TOOLTIP", "ACTION_HIDE_TOOLTIP", AccessibilityAction::HIDE_TOOLTIP), crate::meta::inspect::EnumConstant::new("SHOW_TOOLTIP", "ACTION_SHOW_TOOLTIP", AccessibilityAction::SHOW_TOOLTIP), crate::meta::inspect::EnumConstant::new("SET_TEXT_SELECTION", "ACTION_SET_TEXT_SELECTION", AccessibilityAction::SET_TEXT_SELECTION), crate::meta::inspect::EnumConstant::new("REPLACE_SELECTED_TEXT", "ACTION_REPLACE_SELECTED_TEXT", AccessibilityAction::REPLACE_SELECTED_TEXT), crate::meta::inspect::EnumConstant::new("SCROLL_BACKWARD", "ACTION_SCROLL_BACKWARD", AccessibilityAction::SCROLL_BACKWARD), crate::meta::inspect::EnumConstant::new("SCROLL_DOWN", "ACTION_SCROLL_DOWN", AccessibilityAction::SCROLL_DOWN), crate::meta::inspect::EnumConstant::new("SCROLL_FORWARD", "ACTION_SCROLL_FORWARD", AccessibilityAction::SCROLL_FORWARD), crate::meta::inspect::EnumConstant::new("SCROLL_LEFT", "ACTION_SCROLL_LEFT", AccessibilityAction::SCROLL_LEFT), crate::meta::inspect::EnumConstant::new("SCROLL_RIGHT", "ACTION_SCROLL_RIGHT", AccessibilityAction::SCROLL_RIGHT), crate::meta::inspect::EnumConstant::new("SCROLL_UP", "ACTION_SCROLL_UP", AccessibilityAction::SCROLL_UP), crate::meta::inspect::EnumConstant::new("SCROLL_INTO_VIEW", "ACTION_SCROLL_INTO_VIEW", AccessibilityAction::SCROLL_INTO_VIEW), crate::meta::inspect::EnumConstant::new("SCROLL_TO_POINT", "ACTION_SCROLL_TO_POINT", AccessibilityAction::SCROLL_TO_POINT), crate::meta::inspect::EnumConstant::new("SET_SCROLL_OFFSET", "ACTION_SET_SCROLL_OFFSET", AccessibilityAction::SET_SCROLL_OFFSET), crate::meta::inspect::EnumConstant::new("SET_VALUE", "ACTION_SET_VALUE", AccessibilityAction::SET_VALUE), crate::meta::inspect::EnumConstant::new("SHOW_CONTEXT_MENU", "ACTION_SHOW_CONTEXT_MENU", AccessibilityAction::SHOW_CONTEXT_MENU), crate::meta::inspect::EnumConstant::new("CUSTOM", "ACTION_CUSTOM", AccessibilityAction::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityAction {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AccessibilityAction {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityAction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AccessibilityLiveMode {
    ord: i32
}
impl AccessibilityLiveMode {
    #[doc(alias = "LIVE_OFF")]
    #[doc = "Godot enumerator name: `LIVE_OFF`"]
    pub const OFF: AccessibilityLiveMode = AccessibilityLiveMode {
        ord: 0i32
    };
    #[doc(alias = "LIVE_POLITE")]
    #[doc = "Godot enumerator name: `LIVE_POLITE`"]
    pub const POLITE: AccessibilityLiveMode = AccessibilityLiveMode {
        ord: 1i32
    };
    #[doc(alias = "LIVE_ASSERTIVE")]
    #[doc = "Godot enumerator name: `LIVE_ASSERTIVE`"]
    pub const ASSERTIVE: AccessibilityLiveMode = AccessibilityLiveMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AccessibilityLiveMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityLiveMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityLiveMode {
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
            Self::OFF => "OFF", Self::POLITE => "POLITE", Self::ASSERTIVE => "ASSERTIVE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityLiveMode::OFF, AccessibilityLiveMode::POLITE, AccessibilityLiveMode::ASSERTIVE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityLiveMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OFF", "LIVE_OFF", AccessibilityLiveMode::OFF), crate::meta::inspect::EnumConstant::new("POLITE", "LIVE_POLITE", AccessibilityLiveMode::POLITE), crate::meta::inspect::EnumConstant::new("ASSERTIVE", "LIVE_ASSERTIVE", AccessibilityLiveMode::ASSERTIVE)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityLiveMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AccessibilityLiveMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityLiveMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AccessibilityScrollUnit {
    ord: i32
}
impl AccessibilityScrollUnit {
    #[doc(alias = "SCROLL_UNIT_ITEM")]
    #[doc = "Godot enumerator name: `SCROLL_UNIT_ITEM`"]
    pub const ITEM: AccessibilityScrollUnit = AccessibilityScrollUnit {
        ord: 0i32
    };
    #[doc(alias = "SCROLL_UNIT_PAGE")]
    #[doc = "Godot enumerator name: `SCROLL_UNIT_PAGE`"]
    pub const PAGE: AccessibilityScrollUnit = AccessibilityScrollUnit {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AccessibilityScrollUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityScrollUnit") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityScrollUnit {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::ITEM => "ITEM", Self::PAGE => "PAGE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityScrollUnit::ITEM, AccessibilityScrollUnit::PAGE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityScrollUnit >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ITEM", "SCROLL_UNIT_ITEM", AccessibilityScrollUnit::ITEM), crate::meta::inspect::EnumConstant::new("PAGE", "SCROLL_UNIT_PAGE", AccessibilityScrollUnit::PAGE)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityScrollUnit {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AccessibilityScrollUnit {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityScrollUnit {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AccessibilityScrollHint {
    ord: i32
}
impl AccessibilityScrollHint {
    #[doc(alias = "SCROLL_HINT_TOP_LEFT")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_TOP_LEFT`"]
    pub const TOP_LEFT: AccessibilityScrollHint = AccessibilityScrollHint {
        ord: 0i32
    };
    #[doc(alias = "SCROLL_HINT_BOTTOM_RIGHT")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_BOTTOM_RIGHT`"]
    pub const BOTTOM_RIGHT: AccessibilityScrollHint = AccessibilityScrollHint {
        ord: 1i32
    };
    #[doc(alias = "SCROLL_HINT_TOP_EDGE")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_TOP_EDGE`"]
    pub const TOP_EDGE: AccessibilityScrollHint = AccessibilityScrollHint {
        ord: 2i32
    };
    #[doc(alias = "SCROLL_HINT_BOTTOM_EDGE")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_BOTTOM_EDGE`"]
    pub const BOTTOM_EDGE: AccessibilityScrollHint = AccessibilityScrollHint {
        ord: 3i32
    };
    #[doc(alias = "SCROLL_HINT_LEFT_EDGE")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_LEFT_EDGE`"]
    pub const LEFT_EDGE: AccessibilityScrollHint = AccessibilityScrollHint {
        ord: 4i32
    };
    #[doc(alias = "SCROLL_HINT_RIGHT_EDGE")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_RIGHT_EDGE`"]
    pub const RIGHT_EDGE: AccessibilityScrollHint = AccessibilityScrollHint {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for AccessibilityScrollHint {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityScrollHint") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityScrollHint {
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
            Self::TOP_LEFT => "TOP_LEFT", Self::BOTTOM_RIGHT => "BOTTOM_RIGHT", Self::TOP_EDGE => "TOP_EDGE", Self::BOTTOM_EDGE => "BOTTOM_EDGE", Self::LEFT_EDGE => "LEFT_EDGE", Self::RIGHT_EDGE => "RIGHT_EDGE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityScrollHint::TOP_LEFT, AccessibilityScrollHint::BOTTOM_RIGHT, AccessibilityScrollHint::TOP_EDGE, AccessibilityScrollHint::BOTTOM_EDGE, AccessibilityScrollHint::LEFT_EDGE, AccessibilityScrollHint::RIGHT_EDGE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityScrollHint >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOP_LEFT", "SCROLL_HINT_TOP_LEFT", AccessibilityScrollHint::TOP_LEFT), crate::meta::inspect::EnumConstant::new("BOTTOM_RIGHT", "SCROLL_HINT_BOTTOM_RIGHT", AccessibilityScrollHint::BOTTOM_RIGHT), crate::meta::inspect::EnumConstant::new("TOP_EDGE", "SCROLL_HINT_TOP_EDGE", AccessibilityScrollHint::TOP_EDGE), crate::meta::inspect::EnumConstant::new("BOTTOM_EDGE", "SCROLL_HINT_BOTTOM_EDGE", AccessibilityScrollHint::BOTTOM_EDGE), crate::meta::inspect::EnumConstant::new("LEFT_EDGE", "SCROLL_HINT_LEFT_EDGE", AccessibilityScrollHint::LEFT_EDGE), crate::meta::inspect::EnumConstant::new("RIGHT_EDGE", "SCROLL_HINT_RIGHT_EDGE", AccessibilityScrollHint::RIGHT_EDGE)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityScrollHint {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AccessibilityScrollHint {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityScrollHint {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
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
pub struct ScreenOrientation {
    ord: i32
}
impl ScreenOrientation {
    #[doc(alias = "SCREEN_LANDSCAPE")]
    #[doc = "Godot enumerator name: `SCREEN_LANDSCAPE`"]
    pub const LANDSCAPE: ScreenOrientation = ScreenOrientation {
        ord: 0i32
    };
    #[doc(alias = "SCREEN_PORTRAIT")]
    #[doc = "Godot enumerator name: `SCREEN_PORTRAIT`"]
    pub const PORTRAIT: ScreenOrientation = ScreenOrientation {
        ord: 1i32
    };
    #[doc(alias = "SCREEN_REVERSE_LANDSCAPE")]
    #[doc = "Godot enumerator name: `SCREEN_REVERSE_LANDSCAPE`"]
    pub const REVERSE_LANDSCAPE: ScreenOrientation = ScreenOrientation {
        ord: 2i32
    };
    #[doc(alias = "SCREEN_REVERSE_PORTRAIT")]
    #[doc = "Godot enumerator name: `SCREEN_REVERSE_PORTRAIT`"]
    pub const REVERSE_PORTRAIT: ScreenOrientation = ScreenOrientation {
        ord: 3i32
    };
    #[doc(alias = "SCREEN_SENSOR_LANDSCAPE")]
    #[doc = "Godot enumerator name: `SCREEN_SENSOR_LANDSCAPE`"]
    pub const SENSOR_LANDSCAPE: ScreenOrientation = ScreenOrientation {
        ord: 4i32
    };
    #[doc(alias = "SCREEN_SENSOR_PORTRAIT")]
    #[doc = "Godot enumerator name: `SCREEN_SENSOR_PORTRAIT`"]
    pub const SENSOR_PORTRAIT: ScreenOrientation = ScreenOrientation {
        ord: 5i32
    };
    #[doc(alias = "SCREEN_SENSOR")]
    #[doc = "Godot enumerator name: `SCREEN_SENSOR`"]
    pub const SENSOR: ScreenOrientation = ScreenOrientation {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for ScreenOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ScreenOrientation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ScreenOrientation {
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
            Self::LANDSCAPE => "LANDSCAPE", Self::PORTRAIT => "PORTRAIT", Self::REVERSE_LANDSCAPE => "REVERSE_LANDSCAPE", Self::REVERSE_PORTRAIT => "REVERSE_PORTRAIT", Self::SENSOR_LANDSCAPE => "SENSOR_LANDSCAPE", Self::SENSOR_PORTRAIT => "SENSOR_PORTRAIT", Self::SENSOR => "SENSOR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ScreenOrientation::LANDSCAPE, ScreenOrientation::PORTRAIT, ScreenOrientation::REVERSE_LANDSCAPE, ScreenOrientation::REVERSE_PORTRAIT, ScreenOrientation::SENSOR_LANDSCAPE, ScreenOrientation::SENSOR_PORTRAIT, ScreenOrientation::SENSOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ScreenOrientation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LANDSCAPE", "SCREEN_LANDSCAPE", ScreenOrientation::LANDSCAPE), crate::meta::inspect::EnumConstant::new("PORTRAIT", "SCREEN_PORTRAIT", ScreenOrientation::PORTRAIT), crate::meta::inspect::EnumConstant::new("REVERSE_LANDSCAPE", "SCREEN_REVERSE_LANDSCAPE", ScreenOrientation::REVERSE_LANDSCAPE), crate::meta::inspect::EnumConstant::new("REVERSE_PORTRAIT", "SCREEN_REVERSE_PORTRAIT", ScreenOrientation::REVERSE_PORTRAIT), crate::meta::inspect::EnumConstant::new("SENSOR_LANDSCAPE", "SCREEN_SENSOR_LANDSCAPE", ScreenOrientation::SENSOR_LANDSCAPE), crate::meta::inspect::EnumConstant::new("SENSOR_PORTRAIT", "SCREEN_SENSOR_PORTRAIT", ScreenOrientation::SENSOR_PORTRAIT), crate::meta::inspect::EnumConstant::new("SENSOR", "SCREEN_SENSOR", ScreenOrientation::SENSOR)]
        }
    }
}
impl crate::meta::GodotConvert for ScreenOrientation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ScreenOrientation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ScreenOrientation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VirtualKeyboardType {
    ord: i32
}
impl VirtualKeyboardType {
    #[doc(alias = "KEYBOARD_TYPE_DEFAULT")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_DEFAULT`"]
    pub const DEFAULT: VirtualKeyboardType = VirtualKeyboardType {
        ord: 0i32
    };
    #[doc(alias = "KEYBOARD_TYPE_MULTILINE")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_MULTILINE`"]
    pub const MULTILINE: VirtualKeyboardType = VirtualKeyboardType {
        ord: 1i32
    };
    #[doc(alias = "KEYBOARD_TYPE_NUMBER")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_NUMBER`"]
    pub const NUMBER: VirtualKeyboardType = VirtualKeyboardType {
        ord: 2i32
    };
    #[doc(alias = "KEYBOARD_TYPE_NUMBER_DECIMAL")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_NUMBER_DECIMAL`"]
    pub const NUMBER_DECIMAL: VirtualKeyboardType = VirtualKeyboardType {
        ord: 3i32
    };
    #[doc(alias = "KEYBOARD_TYPE_PHONE")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_PHONE`"]
    pub const PHONE: VirtualKeyboardType = VirtualKeyboardType {
        ord: 4i32
    };
    #[doc(alias = "KEYBOARD_TYPE_EMAIL_ADDRESS")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_EMAIL_ADDRESS`"]
    pub const EMAIL_ADDRESS: VirtualKeyboardType = VirtualKeyboardType {
        ord: 5i32
    };
    #[doc(alias = "KEYBOARD_TYPE_PASSWORD")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_PASSWORD`"]
    pub const PASSWORD: VirtualKeyboardType = VirtualKeyboardType {
        ord: 6i32
    };
    #[doc(alias = "KEYBOARD_TYPE_URL")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_URL`"]
    pub const URL: VirtualKeyboardType = VirtualKeyboardType {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for VirtualKeyboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VirtualKeyboardType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VirtualKeyboardType {
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
            Self::DEFAULT => "DEFAULT", Self::MULTILINE => "MULTILINE", Self::NUMBER => "NUMBER", Self::NUMBER_DECIMAL => "NUMBER_DECIMAL", Self::PHONE => "PHONE", Self::EMAIL_ADDRESS => "EMAIL_ADDRESS", Self::PASSWORD => "PASSWORD", Self::URL => "URL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VirtualKeyboardType::DEFAULT, VirtualKeyboardType::MULTILINE, VirtualKeyboardType::NUMBER, VirtualKeyboardType::NUMBER_DECIMAL, VirtualKeyboardType::PHONE, VirtualKeyboardType::EMAIL_ADDRESS, VirtualKeyboardType::PASSWORD, VirtualKeyboardType::URL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VirtualKeyboardType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "KEYBOARD_TYPE_DEFAULT", VirtualKeyboardType::DEFAULT), crate::meta::inspect::EnumConstant::new("MULTILINE", "KEYBOARD_TYPE_MULTILINE", VirtualKeyboardType::MULTILINE), crate::meta::inspect::EnumConstant::new("NUMBER", "KEYBOARD_TYPE_NUMBER", VirtualKeyboardType::NUMBER), crate::meta::inspect::EnumConstant::new("NUMBER_DECIMAL", "KEYBOARD_TYPE_NUMBER_DECIMAL", VirtualKeyboardType::NUMBER_DECIMAL), crate::meta::inspect::EnumConstant::new("PHONE", "KEYBOARD_TYPE_PHONE", VirtualKeyboardType::PHONE), crate::meta::inspect::EnumConstant::new("EMAIL_ADDRESS", "KEYBOARD_TYPE_EMAIL_ADDRESS", VirtualKeyboardType::EMAIL_ADDRESS), crate::meta::inspect::EnumConstant::new("PASSWORD", "KEYBOARD_TYPE_PASSWORD", VirtualKeyboardType::PASSWORD), crate::meta::inspect::EnumConstant::new("URL", "KEYBOARD_TYPE_URL", VirtualKeyboardType::URL)]
        }
    }
}
impl crate::meta::GodotConvert for VirtualKeyboardType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VirtualKeyboardType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VirtualKeyboardType {
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
    #[doc(alias = "CURSOR_MAX")]
    #[doc = "Godot enumerator name: `CURSOR_MAX`"]
    pub const MAX: CursorShape = CursorShape {
        ord: 17i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 => Some(Self {
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
            Self::ARROW => "ARROW", Self::IBEAM => "IBEAM", Self::POINTING_HAND => "POINTING_HAND", Self::CROSS => "CROSS", Self::WAIT => "WAIT", Self::BUSY => "BUSY", Self::DRAG => "DRAG", Self::CAN_DROP => "CAN_DROP", Self::FORBIDDEN => "FORBIDDEN", Self::VSIZE => "VSIZE", Self::HSIZE => "HSIZE", Self::BDIAGSIZE => "BDIAGSIZE", Self::FDIAGSIZE => "FDIAGSIZE", Self::MOVE => "MOVE", Self::VSPLIT => "VSPLIT", Self::HSPLIT => "HSPLIT", Self::HELP => "HELP", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CursorShape::ARROW, CursorShape::IBEAM, CursorShape::POINTING_HAND, CursorShape::CROSS, CursorShape::WAIT, CursorShape::BUSY, CursorShape::DRAG, CursorShape::CAN_DROP, CursorShape::FORBIDDEN, CursorShape::VSIZE, CursorShape::HSIZE, CursorShape::BDIAGSIZE, CursorShape::FDIAGSIZE, CursorShape::MOVE, CursorShape::VSPLIT, CursorShape::HSPLIT, CursorShape::HELP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CursorShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ARROW", "CURSOR_ARROW", CursorShape::ARROW), crate::meta::inspect::EnumConstant::new("IBEAM", "CURSOR_IBEAM", CursorShape::IBEAM), crate::meta::inspect::EnumConstant::new("POINTING_HAND", "CURSOR_POINTING_HAND", CursorShape::POINTING_HAND), crate::meta::inspect::EnumConstant::new("CROSS", "CURSOR_CROSS", CursorShape::CROSS), crate::meta::inspect::EnumConstant::new("WAIT", "CURSOR_WAIT", CursorShape::WAIT), crate::meta::inspect::EnumConstant::new("BUSY", "CURSOR_BUSY", CursorShape::BUSY), crate::meta::inspect::EnumConstant::new("DRAG", "CURSOR_DRAG", CursorShape::DRAG), crate::meta::inspect::EnumConstant::new("CAN_DROP", "CURSOR_CAN_DROP", CursorShape::CAN_DROP), crate::meta::inspect::EnumConstant::new("FORBIDDEN", "CURSOR_FORBIDDEN", CursorShape::FORBIDDEN), crate::meta::inspect::EnumConstant::new("VSIZE", "CURSOR_VSIZE", CursorShape::VSIZE), crate::meta::inspect::EnumConstant::new("HSIZE", "CURSOR_HSIZE", CursorShape::HSIZE), crate::meta::inspect::EnumConstant::new("BDIAGSIZE", "CURSOR_BDIAGSIZE", CursorShape::BDIAGSIZE), crate::meta::inspect::EnumConstant::new("FDIAGSIZE", "CURSOR_FDIAGSIZE", CursorShape::FDIAGSIZE), crate::meta::inspect::EnumConstant::new("MOVE", "CURSOR_MOVE", CursorShape::MOVE), crate::meta::inspect::EnumConstant::new("VSPLIT", "CURSOR_VSPLIT", CursorShape::VSPLIT), crate::meta::inspect::EnumConstant::new("HSPLIT", "CURSOR_HSPLIT", CursorShape::HSPLIT), crate::meta::inspect::EnumConstant::new("HELP", "CURSOR_HELP", CursorShape::HELP), crate::meta::inspect::EnumConstant::new("MAX", "CURSOR_MAX", CursorShape::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CursorShape {
    const ENUMERATOR_COUNT: usize = 17usize;
    
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FileDialogMode {
    ord: i32
}
impl FileDialogMode {
    #[doc(alias = "FILE_DIALOG_MODE_OPEN_FILE")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_OPEN_FILE`"]
    pub const OPEN_FILE: FileDialogMode = FileDialogMode {
        ord: 0i32
    };
    #[doc(alias = "FILE_DIALOG_MODE_OPEN_FILES")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_OPEN_FILES`"]
    pub const OPEN_FILES: FileDialogMode = FileDialogMode {
        ord: 1i32
    };
    #[doc(alias = "FILE_DIALOG_MODE_OPEN_DIR")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_OPEN_DIR`"]
    pub const OPEN_DIR: FileDialogMode = FileDialogMode {
        ord: 2i32
    };
    #[doc(alias = "FILE_DIALOG_MODE_OPEN_ANY")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_OPEN_ANY`"]
    pub const OPEN_ANY: FileDialogMode = FileDialogMode {
        ord: 3i32
    };
    #[doc(alias = "FILE_DIALOG_MODE_SAVE_FILE")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_SAVE_FILE`"]
    pub const SAVE_FILE: FileDialogMode = FileDialogMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for FileDialogMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FileDialogMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FileDialogMode {
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
            Self::OPEN_FILE => "OPEN_FILE", Self::OPEN_FILES => "OPEN_FILES", Self::OPEN_DIR => "OPEN_DIR", Self::OPEN_ANY => "OPEN_ANY", Self::SAVE_FILE => "SAVE_FILE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FileDialogMode::OPEN_FILE, FileDialogMode::OPEN_FILES, FileDialogMode::OPEN_DIR, FileDialogMode::OPEN_ANY, FileDialogMode::SAVE_FILE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FileDialogMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OPEN_FILE", "FILE_DIALOG_MODE_OPEN_FILE", FileDialogMode::OPEN_FILE), crate::meta::inspect::EnumConstant::new("OPEN_FILES", "FILE_DIALOG_MODE_OPEN_FILES", FileDialogMode::OPEN_FILES), crate::meta::inspect::EnumConstant::new("OPEN_DIR", "FILE_DIALOG_MODE_OPEN_DIR", FileDialogMode::OPEN_DIR), crate::meta::inspect::EnumConstant::new("OPEN_ANY", "FILE_DIALOG_MODE_OPEN_ANY", FileDialogMode::OPEN_ANY), crate::meta::inspect::EnumConstant::new("SAVE_FILE", "FILE_DIALOG_MODE_SAVE_FILE", FileDialogMode::SAVE_FILE)]
        }
    }
}
impl crate::meta::GodotConvert for FileDialogMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FileDialogMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FileDialogMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowMode {
    ord: i32
}
impl WindowMode {
    #[doc(alias = "WINDOW_MODE_WINDOWED")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_WINDOWED`"]
    pub const WINDOWED: WindowMode = WindowMode {
        ord: 0i32
    };
    #[doc(alias = "WINDOW_MODE_MINIMIZED")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_MINIMIZED`"]
    pub const MINIMIZED: WindowMode = WindowMode {
        ord: 1i32
    };
    #[doc(alias = "WINDOW_MODE_MAXIMIZED")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_MAXIMIZED`"]
    pub const MAXIMIZED: WindowMode = WindowMode {
        ord: 2i32
    };
    #[doc(alias = "WINDOW_MODE_FULLSCREEN")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_FULLSCREEN`"]
    pub const FULLSCREEN: WindowMode = WindowMode {
        ord: 3i32
    };
    #[doc(alias = "WINDOW_MODE_EXCLUSIVE_FULLSCREEN")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_EXCLUSIVE_FULLSCREEN`"]
    pub const EXCLUSIVE_FULLSCREEN: WindowMode = WindowMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for WindowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WindowMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WindowMode {
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
            Self::WINDOWED => "WINDOWED", Self::MINIMIZED => "MINIMIZED", Self::MAXIMIZED => "MAXIMIZED", Self::FULLSCREEN => "FULLSCREEN", Self::EXCLUSIVE_FULLSCREEN => "EXCLUSIVE_FULLSCREEN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[WindowMode::WINDOWED, WindowMode::MINIMIZED, WindowMode::MAXIMIZED, WindowMode::FULLSCREEN, WindowMode::EXCLUSIVE_FULLSCREEN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < WindowMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("WINDOWED", "WINDOW_MODE_WINDOWED", WindowMode::WINDOWED), crate::meta::inspect::EnumConstant::new("MINIMIZED", "WINDOW_MODE_MINIMIZED", WindowMode::MINIMIZED), crate::meta::inspect::EnumConstant::new("MAXIMIZED", "WINDOW_MODE_MAXIMIZED", WindowMode::MAXIMIZED), crate::meta::inspect::EnumConstant::new("FULLSCREEN", "WINDOW_MODE_FULLSCREEN", WindowMode::FULLSCREEN), crate::meta::inspect::EnumConstant::new("EXCLUSIVE_FULLSCREEN", "WINDOW_MODE_EXCLUSIVE_FULLSCREEN", WindowMode::EXCLUSIVE_FULLSCREEN)]
        }
    }
}
impl crate::meta::GodotConvert for WindowMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for WindowMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WindowMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowFlags {
    ord: i32
}
impl WindowFlags {
    #[doc(alias = "WINDOW_FLAG_RESIZE_DISABLED")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_RESIZE_DISABLED`"]
    pub const RESIZE_DISABLED: WindowFlags = WindowFlags {
        ord: 0i32
    };
    #[doc(alias = "WINDOW_FLAG_BORDERLESS")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_BORDERLESS`"]
    pub const BORDERLESS: WindowFlags = WindowFlags {
        ord: 1i32
    };
    #[doc(alias = "WINDOW_FLAG_ALWAYS_ON_TOP")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_ALWAYS_ON_TOP`"]
    pub const ALWAYS_ON_TOP: WindowFlags = WindowFlags {
        ord: 2i32
    };
    #[doc(alias = "WINDOW_FLAG_TRANSPARENT")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_TRANSPARENT`"]
    pub const TRANSPARENT: WindowFlags = WindowFlags {
        ord: 3i32
    };
    #[doc(alias = "WINDOW_FLAG_NO_FOCUS")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_NO_FOCUS`"]
    pub const NO_FOCUS: WindowFlags = WindowFlags {
        ord: 4i32
    };
    #[doc(alias = "WINDOW_FLAG_POPUP")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_POPUP`"]
    pub const POPUP: WindowFlags = WindowFlags {
        ord: 5i32
    };
    #[doc(alias = "WINDOW_FLAG_EXTEND_TO_TITLE")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_EXTEND_TO_TITLE`"]
    pub const EXTEND_TO_TITLE: WindowFlags = WindowFlags {
        ord: 6i32
    };
    #[doc(alias = "WINDOW_FLAG_MOUSE_PASSTHROUGH")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_MOUSE_PASSTHROUGH`"]
    pub const MOUSE_PASSTHROUGH: WindowFlags = WindowFlags {
        ord: 7i32
    };
    #[doc(alias = "WINDOW_FLAG_SHARP_CORNERS")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_SHARP_CORNERS`"]
    pub const SHARP_CORNERS: WindowFlags = WindowFlags {
        ord: 8i32
    };
    #[doc(alias = "WINDOW_FLAG_EXCLUDE_FROM_CAPTURE")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_EXCLUDE_FROM_CAPTURE`"]
    pub const EXCLUDE_FROM_CAPTURE: WindowFlags = WindowFlags {
        ord: 9i32
    };
    #[doc(alias = "WINDOW_FLAG_POPUP_WM_HINT")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_POPUP_WM_HINT`"]
    pub const POPUP_WM_HINT: WindowFlags = WindowFlags {
        ord: 10i32
    };
    #[doc(alias = "WINDOW_FLAG_MINIMIZE_DISABLED")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_MINIMIZE_DISABLED`"]
    pub const MINIMIZE_DISABLED: WindowFlags = WindowFlags {
        ord: 11i32
    };
    #[doc(alias = "WINDOW_FLAG_MAXIMIZE_DISABLED")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_MAXIMIZE_DISABLED`"]
    pub const MAXIMIZE_DISABLED: WindowFlags = WindowFlags {
        ord: 12i32
    };
    #[doc(alias = "WINDOW_FLAG_MAX")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_MAX`"]
    pub const MAX: WindowFlags = WindowFlags {
        ord: 13i32
    };
    
}
impl std::fmt::Debug for WindowFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WindowFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WindowFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
            Self::RESIZE_DISABLED => "RESIZE_DISABLED", Self::BORDERLESS => "BORDERLESS", Self::ALWAYS_ON_TOP => "ALWAYS_ON_TOP", Self::TRANSPARENT => "TRANSPARENT", Self::NO_FOCUS => "NO_FOCUS", Self::POPUP => "POPUP", Self::EXTEND_TO_TITLE => "EXTEND_TO_TITLE", Self::MOUSE_PASSTHROUGH => "MOUSE_PASSTHROUGH", Self::SHARP_CORNERS => "SHARP_CORNERS", Self::EXCLUDE_FROM_CAPTURE => "EXCLUDE_FROM_CAPTURE", Self::POPUP_WM_HINT => "POPUP_WM_HINT", Self::MINIMIZE_DISABLED => "MINIMIZE_DISABLED", Self::MAXIMIZE_DISABLED => "MAXIMIZE_DISABLED", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[WindowFlags::RESIZE_DISABLED, WindowFlags::BORDERLESS, WindowFlags::ALWAYS_ON_TOP, WindowFlags::TRANSPARENT, WindowFlags::NO_FOCUS, WindowFlags::POPUP, WindowFlags::EXTEND_TO_TITLE, WindowFlags::MOUSE_PASSTHROUGH, WindowFlags::SHARP_CORNERS, WindowFlags::EXCLUDE_FROM_CAPTURE, WindowFlags::POPUP_WM_HINT, WindowFlags::MINIMIZE_DISABLED, WindowFlags::MAXIMIZE_DISABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < WindowFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RESIZE_DISABLED", "WINDOW_FLAG_RESIZE_DISABLED", WindowFlags::RESIZE_DISABLED), crate::meta::inspect::EnumConstant::new("BORDERLESS", "WINDOW_FLAG_BORDERLESS", WindowFlags::BORDERLESS), crate::meta::inspect::EnumConstant::new("ALWAYS_ON_TOP", "WINDOW_FLAG_ALWAYS_ON_TOP", WindowFlags::ALWAYS_ON_TOP), crate::meta::inspect::EnumConstant::new("TRANSPARENT", "WINDOW_FLAG_TRANSPARENT", WindowFlags::TRANSPARENT), crate::meta::inspect::EnumConstant::new("NO_FOCUS", "WINDOW_FLAG_NO_FOCUS", WindowFlags::NO_FOCUS), crate::meta::inspect::EnumConstant::new("POPUP", "WINDOW_FLAG_POPUP", WindowFlags::POPUP), crate::meta::inspect::EnumConstant::new("EXTEND_TO_TITLE", "WINDOW_FLAG_EXTEND_TO_TITLE", WindowFlags::EXTEND_TO_TITLE), crate::meta::inspect::EnumConstant::new("MOUSE_PASSTHROUGH", "WINDOW_FLAG_MOUSE_PASSTHROUGH", WindowFlags::MOUSE_PASSTHROUGH), crate::meta::inspect::EnumConstant::new("SHARP_CORNERS", "WINDOW_FLAG_SHARP_CORNERS", WindowFlags::SHARP_CORNERS), crate::meta::inspect::EnumConstant::new("EXCLUDE_FROM_CAPTURE", "WINDOW_FLAG_EXCLUDE_FROM_CAPTURE", WindowFlags::EXCLUDE_FROM_CAPTURE), crate::meta::inspect::EnumConstant::new("POPUP_WM_HINT", "WINDOW_FLAG_POPUP_WM_HINT", WindowFlags::POPUP_WM_HINT), crate::meta::inspect::EnumConstant::new("MINIMIZE_DISABLED", "WINDOW_FLAG_MINIMIZE_DISABLED", WindowFlags::MINIMIZE_DISABLED), crate::meta::inspect::EnumConstant::new("MAXIMIZE_DISABLED", "WINDOW_FLAG_MAXIMIZE_DISABLED", WindowFlags::MAXIMIZE_DISABLED), crate::meta::inspect::EnumConstant::new("MAX", "WINDOW_FLAG_MAX", WindowFlags::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for WindowFlags {
    const ENUMERATOR_COUNT: usize = 13usize;
    
}
impl crate::meta::GodotConvert for WindowFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for WindowFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WindowFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowEvent {
    ord: i32
}
impl WindowEvent {
    #[doc(alias = "WINDOW_EVENT_MOUSE_ENTER")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_MOUSE_ENTER`"]
    pub const MOUSE_ENTER: WindowEvent = WindowEvent {
        ord: 0i32
    };
    #[doc(alias = "WINDOW_EVENT_MOUSE_EXIT")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_MOUSE_EXIT`"]
    pub const MOUSE_EXIT: WindowEvent = WindowEvent {
        ord: 1i32
    };
    #[doc(alias = "WINDOW_EVENT_FOCUS_IN")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_FOCUS_IN`"]
    pub const FOCUS_IN: WindowEvent = WindowEvent {
        ord: 2i32
    };
    #[doc(alias = "WINDOW_EVENT_FOCUS_OUT")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_FOCUS_OUT`"]
    pub const FOCUS_OUT: WindowEvent = WindowEvent {
        ord: 3i32
    };
    #[doc(alias = "WINDOW_EVENT_CLOSE_REQUEST")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_CLOSE_REQUEST`"]
    pub const CLOSE_REQUEST: WindowEvent = WindowEvent {
        ord: 4i32
    };
    #[doc(alias = "WINDOW_EVENT_GO_BACK_REQUEST")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_GO_BACK_REQUEST`"]
    pub const GO_BACK_REQUEST: WindowEvent = WindowEvent {
        ord: 5i32
    };
    #[doc(alias = "WINDOW_EVENT_DPI_CHANGE")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_DPI_CHANGE`"]
    pub const DPI_CHANGE: WindowEvent = WindowEvent {
        ord: 6i32
    };
    #[doc(alias = "WINDOW_EVENT_TITLEBAR_CHANGE")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_TITLEBAR_CHANGE`"]
    pub const TITLEBAR_CHANGE: WindowEvent = WindowEvent {
        ord: 7i32
    };
    #[doc(alias = "WINDOW_EVENT_FORCE_CLOSE")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_FORCE_CLOSE`"]
    pub const FORCE_CLOSE: WindowEvent = WindowEvent {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for WindowEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WindowEvent") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WindowEvent {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::MOUSE_ENTER => "MOUSE_ENTER", Self::MOUSE_EXIT => "MOUSE_EXIT", Self::FOCUS_IN => "FOCUS_IN", Self::FOCUS_OUT => "FOCUS_OUT", Self::CLOSE_REQUEST => "CLOSE_REQUEST", Self::GO_BACK_REQUEST => "GO_BACK_REQUEST", Self::DPI_CHANGE => "DPI_CHANGE", Self::TITLEBAR_CHANGE => "TITLEBAR_CHANGE", Self::FORCE_CLOSE => "FORCE_CLOSE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[WindowEvent::MOUSE_ENTER, WindowEvent::MOUSE_EXIT, WindowEvent::FOCUS_IN, WindowEvent::FOCUS_OUT, WindowEvent::CLOSE_REQUEST, WindowEvent::GO_BACK_REQUEST, WindowEvent::DPI_CHANGE, WindowEvent::TITLEBAR_CHANGE, WindowEvent::FORCE_CLOSE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < WindowEvent >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MOUSE_ENTER", "WINDOW_EVENT_MOUSE_ENTER", WindowEvent::MOUSE_ENTER), crate::meta::inspect::EnumConstant::new("MOUSE_EXIT", "WINDOW_EVENT_MOUSE_EXIT", WindowEvent::MOUSE_EXIT), crate::meta::inspect::EnumConstant::new("FOCUS_IN", "WINDOW_EVENT_FOCUS_IN", WindowEvent::FOCUS_IN), crate::meta::inspect::EnumConstant::new("FOCUS_OUT", "WINDOW_EVENT_FOCUS_OUT", WindowEvent::FOCUS_OUT), crate::meta::inspect::EnumConstant::new("CLOSE_REQUEST", "WINDOW_EVENT_CLOSE_REQUEST", WindowEvent::CLOSE_REQUEST), crate::meta::inspect::EnumConstant::new("GO_BACK_REQUEST", "WINDOW_EVENT_GO_BACK_REQUEST", WindowEvent::GO_BACK_REQUEST), crate::meta::inspect::EnumConstant::new("DPI_CHANGE", "WINDOW_EVENT_DPI_CHANGE", WindowEvent::DPI_CHANGE), crate::meta::inspect::EnumConstant::new("TITLEBAR_CHANGE", "WINDOW_EVENT_TITLEBAR_CHANGE", WindowEvent::TITLEBAR_CHANGE), crate::meta::inspect::EnumConstant::new("FORCE_CLOSE", "WINDOW_EVENT_FORCE_CLOSE", WindowEvent::FORCE_CLOSE)]
        }
    }
}
impl crate::meta::GodotConvert for WindowEvent {
    type Via = i32;
    
}
impl crate::meta::ToGodot for WindowEvent {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WindowEvent {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowResizeEdge {
    ord: i32
}
impl WindowResizeEdge {
    #[doc(alias = "WINDOW_EDGE_TOP_LEFT")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_TOP_LEFT`"]
    pub const TOP_LEFT: WindowResizeEdge = WindowResizeEdge {
        ord: 0i32
    };
    #[doc(alias = "WINDOW_EDGE_TOP")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_TOP`"]
    pub const TOP: WindowResizeEdge = WindowResizeEdge {
        ord: 1i32
    };
    #[doc(alias = "WINDOW_EDGE_TOP_RIGHT")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_TOP_RIGHT`"]
    pub const TOP_RIGHT: WindowResizeEdge = WindowResizeEdge {
        ord: 2i32
    };
    #[doc(alias = "WINDOW_EDGE_LEFT")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_LEFT`"]
    pub const LEFT: WindowResizeEdge = WindowResizeEdge {
        ord: 3i32
    };
    #[doc(alias = "WINDOW_EDGE_RIGHT")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_RIGHT`"]
    pub const RIGHT: WindowResizeEdge = WindowResizeEdge {
        ord: 4i32
    };
    #[doc(alias = "WINDOW_EDGE_BOTTOM_LEFT")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_BOTTOM_LEFT`"]
    pub const BOTTOM_LEFT: WindowResizeEdge = WindowResizeEdge {
        ord: 5i32
    };
    #[doc(alias = "WINDOW_EDGE_BOTTOM")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_BOTTOM`"]
    pub const BOTTOM: WindowResizeEdge = WindowResizeEdge {
        ord: 6i32
    };
    #[doc(alias = "WINDOW_EDGE_BOTTOM_RIGHT")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_BOTTOM_RIGHT`"]
    pub const BOTTOM_RIGHT: WindowResizeEdge = WindowResizeEdge {
        ord: 7i32
    };
    #[doc(alias = "WINDOW_EDGE_MAX")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_MAX`"]
    pub const MAX: WindowResizeEdge = WindowResizeEdge {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for WindowResizeEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WindowResizeEdge") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WindowResizeEdge {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::TOP_LEFT => "TOP_LEFT", Self::TOP => "TOP", Self::TOP_RIGHT => "TOP_RIGHT", Self::LEFT => "LEFT", Self::RIGHT => "RIGHT", Self::BOTTOM_LEFT => "BOTTOM_LEFT", Self::BOTTOM => "BOTTOM", Self::BOTTOM_RIGHT => "BOTTOM_RIGHT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[WindowResizeEdge::TOP_LEFT, WindowResizeEdge::TOP, WindowResizeEdge::TOP_RIGHT, WindowResizeEdge::LEFT, WindowResizeEdge::RIGHT, WindowResizeEdge::BOTTOM_LEFT, WindowResizeEdge::BOTTOM, WindowResizeEdge::BOTTOM_RIGHT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < WindowResizeEdge >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOP_LEFT", "WINDOW_EDGE_TOP_LEFT", WindowResizeEdge::TOP_LEFT), crate::meta::inspect::EnumConstant::new("TOP", "WINDOW_EDGE_TOP", WindowResizeEdge::TOP), crate::meta::inspect::EnumConstant::new("TOP_RIGHT", "WINDOW_EDGE_TOP_RIGHT", WindowResizeEdge::TOP_RIGHT), crate::meta::inspect::EnumConstant::new("LEFT", "WINDOW_EDGE_LEFT", WindowResizeEdge::LEFT), crate::meta::inspect::EnumConstant::new("RIGHT", "WINDOW_EDGE_RIGHT", WindowResizeEdge::RIGHT), crate::meta::inspect::EnumConstant::new("BOTTOM_LEFT", "WINDOW_EDGE_BOTTOM_LEFT", WindowResizeEdge::BOTTOM_LEFT), crate::meta::inspect::EnumConstant::new("BOTTOM", "WINDOW_EDGE_BOTTOM", WindowResizeEdge::BOTTOM), crate::meta::inspect::EnumConstant::new("BOTTOM_RIGHT", "WINDOW_EDGE_BOTTOM_RIGHT", WindowResizeEdge::BOTTOM_RIGHT), crate::meta::inspect::EnumConstant::new("MAX", "WINDOW_EDGE_MAX", WindowResizeEdge::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for WindowResizeEdge {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for WindowResizeEdge {
    type Via = i32;
    
}
impl crate::meta::ToGodot for WindowResizeEdge {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WindowResizeEdge {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VSyncMode {
    ord: i32
}
impl VSyncMode {
    #[doc(alias = "VSYNC_DISABLED")]
    #[doc = "Godot enumerator name: `VSYNC_DISABLED`"]
    pub const DISABLED: VSyncMode = VSyncMode {
        ord: 0i32
    };
    #[doc(alias = "VSYNC_ENABLED")]
    #[doc = "Godot enumerator name: `VSYNC_ENABLED`"]
    pub const ENABLED: VSyncMode = VSyncMode {
        ord: 1i32
    };
    #[doc(alias = "VSYNC_ADAPTIVE")]
    #[doc = "Godot enumerator name: `VSYNC_ADAPTIVE`"]
    pub const ADAPTIVE: VSyncMode = VSyncMode {
        ord: 2i32
    };
    #[doc(alias = "VSYNC_MAILBOX")]
    #[doc = "Godot enumerator name: `VSYNC_MAILBOX`"]
    pub const MAILBOX: VSyncMode = VSyncMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for VSyncMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VSyncMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VSyncMode {
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
            Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::ADAPTIVE => "ADAPTIVE", Self::MAILBOX => "MAILBOX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VSyncMode::DISABLED, VSyncMode::ENABLED, VSyncMode::ADAPTIVE, VSyncMode::MAILBOX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VSyncMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VSYNC_DISABLED", VSyncMode::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "VSYNC_ENABLED", VSyncMode::ENABLED), crate::meta::inspect::EnumConstant::new("ADAPTIVE", "VSYNC_ADAPTIVE", VSyncMode::ADAPTIVE), crate::meta::inspect::EnumConstant::new("MAILBOX", "VSYNC_MAILBOX", VSyncMode::MAILBOX)]
        }
    }
}
impl crate::meta::GodotConvert for VSyncMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VSyncMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VSyncMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HandleType {
    ord: i32
}
impl HandleType {
    pub const DISPLAY_HANDLE: HandleType = HandleType {
        ord: 0i32
    };
    pub const WINDOW_HANDLE: HandleType = HandleType {
        ord: 1i32
    };
    pub const WINDOW_VIEW: HandleType = HandleType {
        ord: 2i32
    };
    pub const OPENGL_CONTEXT: HandleType = HandleType {
        ord: 3i32
    };
    pub const EGL_DISPLAY: HandleType = HandleType {
        ord: 4i32
    };
    pub const EGL_CONFIG: HandleType = HandleType {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for HandleType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HandleType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HandleType {
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
            Self::DISPLAY_HANDLE => "DISPLAY_HANDLE", Self::WINDOW_HANDLE => "WINDOW_HANDLE", Self::WINDOW_VIEW => "WINDOW_VIEW", Self::OPENGL_CONTEXT => "OPENGL_CONTEXT", Self::EGL_DISPLAY => "EGL_DISPLAY", Self::EGL_CONFIG => "EGL_CONFIG", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[HandleType::DISPLAY_HANDLE, HandleType::WINDOW_HANDLE, HandleType::WINDOW_VIEW, HandleType::OPENGL_CONTEXT, HandleType::EGL_DISPLAY, HandleType::EGL_CONFIG]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < HandleType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISPLAY_HANDLE", "DISPLAY_HANDLE", HandleType::DISPLAY_HANDLE), crate::meta::inspect::EnumConstant::new("WINDOW_HANDLE", "WINDOW_HANDLE", HandleType::WINDOW_HANDLE), crate::meta::inspect::EnumConstant::new("WINDOW_VIEW", "WINDOW_VIEW", HandleType::WINDOW_VIEW), crate::meta::inspect::EnumConstant::new("OPENGL_CONTEXT", "OPENGL_CONTEXT", HandleType::OPENGL_CONTEXT), crate::meta::inspect::EnumConstant::new("EGL_DISPLAY", "EGL_DISPLAY", HandleType::EGL_DISPLAY), crate::meta::inspect::EnumConstant::new("EGL_CONFIG", "EGL_CONFIG", HandleType::EGL_CONFIG)]
        }
    }
}
impl crate::meta::GodotConvert for HandleType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for HandleType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandleType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `TTSUtteranceEvent`."]
pub struct TtsUtteranceEvent {
    ord: i32
}
impl TtsUtteranceEvent {
    #[doc(alias = "TTS_UTTERANCE_STARTED")]
    #[doc = "Godot enumerator name: `TTS_UTTERANCE_STARTED`"]
    pub const STARTED: TtsUtteranceEvent = TtsUtteranceEvent {
        ord: 0i32
    };
    #[doc(alias = "TTS_UTTERANCE_ENDED")]
    #[doc = "Godot enumerator name: `TTS_UTTERANCE_ENDED`"]
    pub const ENDED: TtsUtteranceEvent = TtsUtteranceEvent {
        ord: 1i32
    };
    #[doc(alias = "TTS_UTTERANCE_CANCELED")]
    #[doc = "Godot enumerator name: `TTS_UTTERANCE_CANCELED`"]
    pub const CANCELED: TtsUtteranceEvent = TtsUtteranceEvent {
        ord: 2i32
    };
    #[doc(alias = "TTS_UTTERANCE_BOUNDARY")]
    #[doc = "Godot enumerator name: `TTS_UTTERANCE_BOUNDARY`"]
    pub const BOUNDARY: TtsUtteranceEvent = TtsUtteranceEvent {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TtsUtteranceEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TtsUtteranceEvent") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TtsUtteranceEvent {
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
            Self::STARTED => "STARTED", Self::ENDED => "ENDED", Self::CANCELED => "CANCELED", Self::BOUNDARY => "BOUNDARY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TtsUtteranceEvent::STARTED, TtsUtteranceEvent::ENDED, TtsUtteranceEvent::CANCELED, TtsUtteranceEvent::BOUNDARY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TtsUtteranceEvent >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STARTED", "TTS_UTTERANCE_STARTED", TtsUtteranceEvent::STARTED), crate::meta::inspect::EnumConstant::new("ENDED", "TTS_UTTERANCE_ENDED", TtsUtteranceEvent::ENDED), crate::meta::inspect::EnumConstant::new("CANCELED", "TTS_UTTERANCE_CANCELED", TtsUtteranceEvent::CANCELED), crate::meta::inspect::EnumConstant::new("BOUNDARY", "TTS_UTTERANCE_BOUNDARY", TtsUtteranceEvent::BOUNDARY)]
        }
    }
}
impl crate::meta::GodotConvert for TtsUtteranceEvent {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TtsUtteranceEvent {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TtsUtteranceEvent {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::DisplayServer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for DisplayServer {
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