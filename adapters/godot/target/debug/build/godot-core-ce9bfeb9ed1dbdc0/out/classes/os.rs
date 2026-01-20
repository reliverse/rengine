#![doc = "Sidecar module for class [`Os`][crate::classes::Os].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OS` enums](https://docs.godotengine.org/en/stable/classes/class_os.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OS.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`os`][crate::classes::os]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `OS`](https://docs.godotengine.org/en/stable/classes/class_os.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Os {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Os {
        pub fn get_entropy(&mut self, size: i32,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(43usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_entropy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_system_ca_certificates(&mut self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(44usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_system_ca_certificates", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_connected_midi_inputs(&mut self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(45usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_connected_midi_inputs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn open_midi_inputs(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(46usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "open_midi_inputs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn close_midi_inputs(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(47usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "close_midi_inputs", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn alert_full(&mut self, text: CowArg < GString >, title: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (text, title,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(48usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "alert", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::alert_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn alert(&mut self, text: impl AsArg < GString >,) {
            self.alert_ex(text,) . done()
        }
        #[inline]
        pub fn alert_ex < 'a > (&'a mut self, text: impl AsArg < GString > + 'a,) -> ExAlert < 'a > {
            ExAlert::new(self, text,)
        }
        pub fn crash(&mut self, message: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (message.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(49usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "crash", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_low_processor_usage_mode(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(50usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_low_processor_usage_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_in_low_processor_usage_mode(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(51usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_in_low_processor_usage_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_low_processor_usage_mode_sleep_usec(&mut self, usec: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (usec,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(52usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_low_processor_usage_mode_sleep_usec", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_low_processor_usage_mode_sleep_usec(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(53usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_low_processor_usage_mode_sleep_usec", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_delta_smoothing(&mut self, delta_smoothing_enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (delta_smoothing_enabled,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(54usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_delta_smoothing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_delta_smoothing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(55usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_delta_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_processor_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(56usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_processor_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_processor_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(57usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_processor_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_system_fonts(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(58usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_system_fonts", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_system_font_path_full(&self, font_name: CowArg < GString >, weight: i32, stretch: i32, italic: bool,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32, bool,);
            let args = (font_name, weight, stretch, italic,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(59usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_system_font_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_system_font_path_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_system_font_path(&self, font_name: impl AsArg < GString >,) -> GString {
            self.get_system_font_path_ex(font_name,) . done()
        }
        #[inline]
        pub fn get_system_font_path_ex < 'a > (&'a self, font_name: impl AsArg < GString > + 'a,) -> ExGetSystemFontPath < 'a > {
            ExGetSystemFontPath::new(self, font_name,)
        }
        pub(crate) fn get_system_font_path_for_text_full(&self, font_name: CowArg < GString >, text: CowArg < GString >, locale: CowArg < GString >, script: CowArg < GString >, weight: i32, stretch: i32, italic: bool,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, CowArg < 'a3, GString >, i32, i32, bool,);
            let args = (font_name, text, locale, script, weight, stretch, italic,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(60usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_system_font_path_for_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_system_font_path_for_text_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_system_font_path_for_text(&self, font_name: impl AsArg < GString >, text: impl AsArg < GString >,) -> PackedStringArray {
            self.get_system_font_path_for_text_ex(font_name, text,) . done()
        }
        #[inline]
        pub fn get_system_font_path_for_text_ex < 'a > (&'a self, font_name: impl AsArg < GString > + 'a, text: impl AsArg < GString > + 'a,) -> ExGetSystemFontPathForText < 'a > {
            ExGetSystemFontPathForText::new(self, font_name, text,)
        }
        pub fn get_executable_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(61usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_executable_path", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn read_string_from_stdin_full(&mut self, buffer_size: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(62usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "read_string_from_stdin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::read_string_from_stdin_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn read_string_from_stdin(&mut self,) -> GString {
            self.read_string_from_stdin_ex() . done()
        }
        #[inline]
        pub fn read_string_from_stdin_ex < 'a > (&'a mut self,) -> ExReadStringFromStdin < 'a > {
            ExReadStringFromStdin::new(self,)
        }
        pub(crate) fn read_buffer_from_stdin_full(&mut self, buffer_size: i64,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (i64,);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(63usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "read_buffer_from_stdin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::read_buffer_from_stdin_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn read_buffer_from_stdin(&mut self,) -> PackedByteArray {
            self.read_buffer_from_stdin_ex() . done()
        }
        #[inline]
        pub fn read_buffer_from_stdin_ex < 'a > (&'a mut self,) -> ExReadBufferFromStdin < 'a > {
            ExReadBufferFromStdin::new(self,)
        }
        pub fn get_stdin_type(&self,) -> crate::classes::os::StdHandleType {
            type CallRet = crate::classes::os::StdHandleType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(64usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_stdin_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stdout_type(&self,) -> crate::classes::os::StdHandleType {
            type CallRet = crate::classes::os::StdHandleType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(65usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_stdout_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stderr_type(&self,) -> crate::classes::os::StdHandleType {
            type CallRet = crate::classes::os::StdHandleType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(66usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_stderr_type", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn execute_full(&mut self, path: CowArg < GString >, arguments: RefArg < PackedStringArray >, output: RefArg < VarArray >, read_stderr: bool, open_console: bool,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, RefArg < 'a2, VarArray >, bool, bool,);
            let args = (path, arguments, output, read_stderr, open_console,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(67usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "execute", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::execute_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn execute(&mut self, path: impl AsArg < GString >, arguments: &PackedStringArray,) -> i32 {
            self.execute_ex(path, arguments,) . done()
        }
        #[inline]
        pub fn execute_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a, arguments: &'a PackedStringArray,) -> ExExecute < 'a > {
            ExExecute::new(self, path, arguments,)
        }
        pub(crate) fn execute_with_pipe_full(&mut self, path: CowArg < GString >, arguments: RefArg < PackedStringArray >, blocking: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, bool,);
            let args = (path, arguments, blocking,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(68usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "execute_with_pipe", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::execute_with_pipe_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn execute_with_pipe(&mut self, path: impl AsArg < GString >, arguments: &PackedStringArray,) -> VarDictionary {
            self.execute_with_pipe_ex(path, arguments,) . done()
        }
        #[inline]
        pub fn execute_with_pipe_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a, arguments: &'a PackedStringArray,) -> ExExecuteWithPipe < 'a > {
            ExExecuteWithPipe::new(self, path, arguments,)
        }
        pub(crate) fn create_process_full(&mut self, path: CowArg < GString >, arguments: RefArg < PackedStringArray >, open_console: bool,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, bool,);
            let args = (path, arguments, open_console,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(69usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "create_process", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_process_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_process(&mut self, path: impl AsArg < GString >, arguments: &PackedStringArray,) -> i32 {
            self.create_process_ex(path, arguments,) . done()
        }
        #[inline]
        pub fn create_process_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a, arguments: &'a PackedStringArray,) -> ExCreateProcess < 'a > {
            ExCreateProcess::new(self, path, arguments,)
        }
        pub fn create_instance(&mut self, arguments: &PackedStringArray,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(arguments),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(70usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "create_instance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn open_with_program(&mut self, program_path: impl AsArg < GString >, paths: &PackedStringArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >,);
            let args = (program_path.into_arg(), RefArg::new(paths),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(71usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "open_with_program", Some(self.__validated_obj()), args,)
            }
        }
        pub fn kill(&mut self, pid: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (i32,);
            let args = (pid,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(72usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "kill", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shell_open(&mut self, uri: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (uri.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(73usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "shell_open", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn shell_show_in_file_manager_full(&mut self, file_or_dir_path: CowArg < GString >, open_folder: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (file_or_dir_path, open_folder,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(74usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "shell_show_in_file_manager", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shell_show_in_file_manager_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shell_show_in_file_manager(&mut self, file_or_dir_path: impl AsArg < GString >,) -> crate::global::Error {
            self.shell_show_in_file_manager_ex(file_or_dir_path,) . done()
        }
        #[inline]
        pub fn shell_show_in_file_manager_ex < 'a > (&'a mut self, file_or_dir_path: impl AsArg < GString > + 'a,) -> ExShellShowInFileManager < 'a > {
            ExShellShowInFileManager::new(self, file_or_dir_path,)
        }
        pub fn is_process_running(&self, pid: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (pid,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(75usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_process_running", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_exit_code(&self, pid: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (pid,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(76usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_process_exit_code", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(77usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_process_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_environment(&self, variable: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (variable.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(78usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "has_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_environment(&self, variable: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (variable.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(79usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_environment(&self, variable: impl AsArg < GString >, value: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (variable.into_arg(), value.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(80usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unset_environment(&self, variable: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (variable.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(81usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "unset_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(82usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_distribution_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(83usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_distribution_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_version(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(84usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_version_alias(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(85usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_version_alias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cmdline_args(&mut self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(86usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_cmdline_args", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cmdline_user_args(&mut self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(87usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_cmdline_user_args", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_video_adapter_driver_info(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(88usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_video_adapter_driver_info", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_restart_on_exit_full(&mut self, restart: bool, arguments: RefArg < PackedStringArray >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (bool, RefArg < 'a0, PackedStringArray >,);
            let args = (restart, arguments,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(89usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_restart_on_exit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_restart_on_exit_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_restart_on_exit(&mut self, restart: bool,) {
            self.set_restart_on_exit_ex(restart,) . done()
        }
        #[inline]
        pub fn set_restart_on_exit_ex < 'a > (&'a mut self, restart: bool,) -> ExSetRestartOnExit < 'a > {
            ExSetRestartOnExit::new(self, restart,)
        }
        pub fn is_restart_on_exit_set(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(90usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_restart_on_exit_set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_restart_on_exit_arguments(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(91usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_restart_on_exit_arguments", Some(self.__validated_obj()), args,)
            }
        }
        pub fn delay_usec(&self, usec: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (usec,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(92usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "delay_usec", Some(self.__validated_obj()), args,)
            }
        }
        pub fn delay_msec(&self, msec: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (msec,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(93usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "delay_msec", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_locale(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(94usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_locale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_locale_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(95usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_locale_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_model_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(96usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_model_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_userfs_persistent(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(97usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_userfs_persistent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_stdout_verbose(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(98usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_stdout_verbose", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_debug_build(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(99usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_debug_build", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_static_memory_usage(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_static_memory_usage", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_static_memory_peak_usage(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_static_memory_peak_usage", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_memory_info(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_memory_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn move_to_trash(&self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "move_to_trash", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_user_data_dir(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_user_data_dir", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_system_dir_full(&self, dir: crate::classes::os::SystemDir, shared_storage: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (crate::classes::os::SystemDir, bool,);
            let args = (dir, shared_storage,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_system_dir", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_system_dir_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_system_dir(&self, dir: crate::classes::os::SystemDir,) -> GString {
            self.get_system_dir_ex(dir,) . done()
        }
        #[inline]
        pub fn get_system_dir_ex < 'a > (&'a self, dir: crate::classes::os::SystemDir,) -> ExGetSystemDir < 'a > {
            ExGetSystemDir::new(self, dir,)
        }
        pub fn get_config_dir(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_config_dir", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_data_dir(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_data_dir", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cache_dir(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_cache_dir", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_temp_dir(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_temp_dir", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_unique_id(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_unique_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keycode_string(&self, code: crate::global::Key,) -> GString {
            type CallRet = GString;
            type CallParams = (crate::global::Key,);
            let args = (code,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_keycode_string", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_keycode_unicode(&self, code: u32,) -> bool {
            type CallRet = bool;
            type CallParams = (u32,);
            let args = (code,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_keycode_unicode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn find_keycode_from_string(&self, string: impl AsArg < GString >,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "find_keycode_from_string", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_file_access_save_and_swap(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_use_file_access_save_and_swap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_thread_name(&mut self, name: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_thread_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_thread_caller_id(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_thread_caller_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_main_thread_id(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_main_thread_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_feature(&self, tag_name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (tag_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "has_feature", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_sandboxed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_sandboxed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn request_permission(&mut self, name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "request_permission", Some(self.__validated_obj()), args,)
            }
        }
        pub fn request_permissions(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "request_permissions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_granted_permissions(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_granted_permissions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn revoke_granted_permissions(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "revoke_granted_permissions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_logger(&mut self, logger: impl AsArg < Option < Gd < crate::classes::Logger >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Logger > > >,);
            let args = (logger.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "add_logger", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_logger(&mut self, logger: impl AsArg < Option < Gd < crate::classes::Logger >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Logger > > >,);
            let args = (logger.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "remove_logger", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Os {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OS"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Core;
        
    }
    unsafe impl crate::obj::Bounds for Os {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Os {
        
    }
    impl crate::obj::Singleton for Os {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"OS"))
            }
        }
    }
    impl std::ops::Deref for Os {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Os {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Os__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Os` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Os::alert_ex`][super::Os::alert_ex]."]
#[must_use]
pub struct ExAlert < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Os, text: CowArg < 'a, GString >, title: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAlert < 'a > {
    fn new(surround_object: &'a mut re_export::Os, text: impl AsArg < GString > + 'a,) -> Self {
        let title = GString::from("Alert!");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), title: CowArg::Owned(title),
        }
    }
    #[inline]
    pub fn title(self, title: impl AsArg < GString > + 'a) -> Self {
        Self {
            title: title.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, text, title,
        }
        = self;
        re_export::Os::alert_full(surround_object, text, title,)
    }
}
#[doc = "Default-param extender for [`Os::get_system_font_path_ex`][super::Os::get_system_font_path_ex]."]
#[must_use]
pub struct ExGetSystemFontPath < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Os, font_name: CowArg < 'a, GString >, weight: i32, stretch: i32, italic: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSystemFontPath < 'a > {
    fn new(surround_object: &'a re_export::Os, font_name: impl AsArg < GString > + 'a,) -> Self {
        let weight = 400i32;
        let stretch = 100i32;
        let italic = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_name: font_name.into_arg(), weight: weight, stretch: stretch, italic: italic,
        }
    }
    #[inline]
    pub fn weight(self, weight: i32) -> Self {
        Self {
            weight: weight, .. self
        }
    }
    #[inline]
    pub fn stretch(self, stretch: i32) -> Self {
        Self {
            stretch: stretch, .. self
        }
    }
    #[inline]
    pub fn italic(self, italic: bool) -> Self {
        Self {
            italic: italic, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, font_name, weight, stretch, italic,
        }
        = self;
        re_export::Os::get_system_font_path_full(surround_object, font_name, weight, stretch, italic,)
    }
}
#[doc = "Default-param extender for [`Os::get_system_font_path_for_text_ex`][super::Os::get_system_font_path_for_text_ex]."]
#[must_use]
pub struct ExGetSystemFontPathForText < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Os, font_name: CowArg < 'a, GString >, text: CowArg < 'a, GString >, locale: CowArg < 'a, GString >, script: CowArg < 'a, GString >, weight: i32, stretch: i32, italic: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSystemFontPathForText < 'a > {
    fn new(surround_object: &'a re_export::Os, font_name: impl AsArg < GString > + 'a, text: impl AsArg < GString > + 'a,) -> Self {
        let locale = GString::from("");
        let script = GString::from("");
        let weight = 400i32;
        let stretch = 100i32;
        let italic = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_name: font_name.into_arg(), text: text.into_arg(), locale: CowArg::Owned(locale), script: CowArg::Owned(script), weight: weight, stretch: stretch, italic: italic,
        }
    }
    #[inline]
    pub fn locale(self, locale: impl AsArg < GString > + 'a) -> Self {
        Self {
            locale: locale.into_arg(), .. self
        }
    }
    #[inline]
    pub fn script(self, script: impl AsArg < GString > + 'a) -> Self {
        Self {
            script: script.into_arg(), .. self
        }
    }
    #[inline]
    pub fn weight(self, weight: i32) -> Self {
        Self {
            weight: weight, .. self
        }
    }
    #[inline]
    pub fn stretch(self, stretch: i32) -> Self {
        Self {
            stretch: stretch, .. self
        }
    }
    #[inline]
    pub fn italic(self, italic: bool) -> Self {
        Self {
            italic: italic, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, font_name, text, locale, script, weight, stretch, italic,
        }
        = self;
        re_export::Os::get_system_font_path_for_text_full(surround_object, font_name, text, locale, script, weight, stretch, italic,)
    }
}
#[doc = "Default-param extender for [`Os::read_string_from_stdin_ex`][super::Os::read_string_from_stdin_ex]."]
#[must_use]
pub struct ExReadStringFromStdin < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Os, buffer_size: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExReadStringFromStdin < 'a > {
    fn new(surround_object: &'a mut re_export::Os,) -> Self {
        let buffer_size = 1024i64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, buffer_size: buffer_size,
        }
    }
    #[inline]
    pub fn buffer_size(self, buffer_size: i64) -> Self {
        Self {
            buffer_size: buffer_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, buffer_size,
        }
        = self;
        re_export::Os::read_string_from_stdin_full(surround_object, buffer_size,)
    }
}
#[doc = "Default-param extender for [`Os::read_buffer_from_stdin_ex`][super::Os::read_buffer_from_stdin_ex]."]
#[must_use]
pub struct ExReadBufferFromStdin < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Os, buffer_size: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExReadBufferFromStdin < 'a > {
    fn new(surround_object: &'a mut re_export::Os,) -> Self {
        let buffer_size = 1024i64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, buffer_size: buffer_size,
        }
    }
    #[inline]
    pub fn buffer_size(self, buffer_size: i64) -> Self {
        Self {
            buffer_size: buffer_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        let Self {
            _phantom, surround_object, buffer_size,
        }
        = self;
        re_export::Os::read_buffer_from_stdin_full(surround_object, buffer_size,)
    }
}
#[doc = "Default-param extender for [`Os::execute_ex`][super::Os::execute_ex]."]
#[must_use]
pub struct ExExecute < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Os, path: CowArg < 'a, GString >, arguments: CowArg < 'a, PackedStringArray >, output: CowArg < 'a, VarArray >, read_stderr: bool, open_console: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExExecute < 'a > {
    fn new(surround_object: &'a mut re_export::Os, path: impl AsArg < GString > + 'a, arguments: &'a PackedStringArray,) -> Self {
        let output = Array::new();
        let read_stderr = false;
        let open_console = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), arguments: CowArg::Borrowed(arguments), output: CowArg::Owned(output), read_stderr: read_stderr, open_console: open_console,
        }
    }
    #[inline]
    pub fn output(self, output: &'a VarArray) -> Self {
        Self {
            output: CowArg::Borrowed(output), .. self
        }
    }
    #[inline]
    pub fn read_stderr(self, read_stderr: bool) -> Self {
        Self {
            read_stderr: read_stderr, .. self
        }
    }
    #[inline]
    pub fn open_console(self, open_console: bool) -> Self {
        Self {
            open_console: open_console, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, path, arguments, output, read_stderr, open_console,
        }
        = self;
        re_export::Os::execute_full(surround_object, path, arguments.cow_as_arg(), output.cow_as_arg(), read_stderr, open_console,)
    }
}
#[doc = "Default-param extender for [`Os::execute_with_pipe_ex`][super::Os::execute_with_pipe_ex]."]
#[must_use]
pub struct ExExecuteWithPipe < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Os, path: CowArg < 'a, GString >, arguments: CowArg < 'a, PackedStringArray >, blocking: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExExecuteWithPipe < 'a > {
    fn new(surround_object: &'a mut re_export::Os, path: impl AsArg < GString > + 'a, arguments: &'a PackedStringArray,) -> Self {
        let blocking = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), arguments: CowArg::Borrowed(arguments), blocking: blocking,
        }
    }
    #[inline]
    pub fn blocking(self, blocking: bool) -> Self {
        Self {
            blocking: blocking, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarDictionary {
        let Self {
            _phantom, surround_object, path, arguments, blocking,
        }
        = self;
        re_export::Os::execute_with_pipe_full(surround_object, path, arguments.cow_as_arg(), blocking,)
    }
}
#[doc = "Default-param extender for [`Os::create_process_ex`][super::Os::create_process_ex]."]
#[must_use]
pub struct ExCreateProcess < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Os, path: CowArg < 'a, GString >, arguments: CowArg < 'a, PackedStringArray >, open_console: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateProcess < 'a > {
    fn new(surround_object: &'a mut re_export::Os, path: impl AsArg < GString > + 'a, arguments: &'a PackedStringArray,) -> Self {
        let open_console = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), arguments: CowArg::Borrowed(arguments), open_console: open_console,
        }
    }
    #[inline]
    pub fn open_console(self, open_console: bool) -> Self {
        Self {
            open_console: open_console, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, path, arguments, open_console,
        }
        = self;
        re_export::Os::create_process_full(surround_object, path, arguments.cow_as_arg(), open_console,)
    }
}
#[doc = "Default-param extender for [`Os::shell_show_in_file_manager_ex`][super::Os::shell_show_in_file_manager_ex]."]
#[must_use]
pub struct ExShellShowInFileManager < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Os, file_or_dir_path: CowArg < 'a, GString >, open_folder: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShellShowInFileManager < 'a > {
    fn new(surround_object: &'a mut re_export::Os, file_or_dir_path: impl AsArg < GString > + 'a,) -> Self {
        let open_folder = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, file_or_dir_path: file_or_dir_path.into_arg(), open_folder: open_folder,
        }
    }
    #[inline]
    pub fn open_folder(self, open_folder: bool) -> Self {
        Self {
            open_folder: open_folder, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, file_or_dir_path, open_folder,
        }
        = self;
        re_export::Os::shell_show_in_file_manager_full(surround_object, file_or_dir_path, open_folder,)
    }
}
#[doc = "Default-param extender for [`Os::set_restart_on_exit_ex`][super::Os::set_restart_on_exit_ex]."]
#[must_use]
pub struct ExSetRestartOnExit < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Os, restart: bool, arguments: CowArg < 'a, PackedStringArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetRestartOnExit < 'a > {
    fn new(surround_object: &'a mut re_export::Os, restart: bool,) -> Self {
        let arguments = PackedStringArray::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, restart: restart, arguments: CowArg::Owned(arguments),
        }
    }
    #[inline]
    pub fn arguments(self, arguments: &'a PackedStringArray) -> Self {
        Self {
            arguments: CowArg::Borrowed(arguments), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, restart, arguments,
        }
        = self;
        re_export::Os::set_restart_on_exit_full(surround_object, restart, arguments.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`Os::get_system_dir_ex`][super::Os::get_system_dir_ex]."]
#[must_use]
pub struct ExGetSystemDir < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Os, dir: crate::classes::os::SystemDir, shared_storage: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSystemDir < 'a > {
    fn new(surround_object: &'a re_export::Os, dir: crate::classes::os::SystemDir,) -> Self {
        let shared_storage = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dir: dir, shared_storage: shared_storage,
        }
    }
    #[inline]
    pub fn shared_storage(self, shared_storage: bool) -> Self {
        Self {
            shared_storage: shared_storage, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, dir, shared_storage,
        }
        = self;
        re_export::Os::get_system_dir_full(surround_object, dir, shared_storage,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderingDriver {
    ord: i32
}
impl RenderingDriver {
    #[doc(alias = "RENDERING_DRIVER_VULKAN")]
    #[doc = "Godot enumerator name: `RENDERING_DRIVER_VULKAN`"]
    pub const VULKAN: RenderingDriver = RenderingDriver {
        ord: 0i32
    };
    #[doc(alias = "RENDERING_DRIVER_OPENGL3")]
    #[doc = "Godot enumerator name: `RENDERING_DRIVER_OPENGL3`"]
    pub const OPENGL3: RenderingDriver = RenderingDriver {
        ord: 1i32
    };
    #[doc(alias = "RENDERING_DRIVER_D3D12")]
    #[doc = "Godot enumerator name: `RENDERING_DRIVER_D3D12`"]
    pub const D3D12: RenderingDriver = RenderingDriver {
        ord: 2i32
    };
    #[doc(alias = "RENDERING_DRIVER_METAL")]
    #[doc = "Godot enumerator name: `RENDERING_DRIVER_METAL`"]
    pub const METAL: RenderingDriver = RenderingDriver {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for RenderingDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RenderingDriver") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RenderingDriver {
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
            Self::VULKAN => "VULKAN", Self::OPENGL3 => "OPENGL3", Self::D3D12 => "D3D12", Self::METAL => "METAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[RenderingDriver::VULKAN, RenderingDriver::OPENGL3, RenderingDriver::D3D12, RenderingDriver::METAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < RenderingDriver >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VULKAN", "RENDERING_DRIVER_VULKAN", RenderingDriver::VULKAN), crate::meta::inspect::EnumConstant::new("OPENGL3", "RENDERING_DRIVER_OPENGL3", RenderingDriver::OPENGL3), crate::meta::inspect::EnumConstant::new("D3D12", "RENDERING_DRIVER_D3D12", RenderingDriver::D3D12), crate::meta::inspect::EnumConstant::new("METAL", "RENDERING_DRIVER_METAL", RenderingDriver::METAL)]
        }
    }
}
impl crate::meta::GodotConvert for RenderingDriver {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RenderingDriver {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RenderingDriver {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SystemDir {
    ord: i32
}
impl SystemDir {
    #[doc(alias = "SYSTEM_DIR_DESKTOP")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_DESKTOP`"]
    pub const DESKTOP: SystemDir = SystemDir {
        ord: 0i32
    };
    #[doc(alias = "SYSTEM_DIR_DCIM")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_DCIM`"]
    pub const DCIM: SystemDir = SystemDir {
        ord: 1i32
    };
    #[doc(alias = "SYSTEM_DIR_DOCUMENTS")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_DOCUMENTS`"]
    pub const DOCUMENTS: SystemDir = SystemDir {
        ord: 2i32
    };
    #[doc(alias = "SYSTEM_DIR_DOWNLOADS")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_DOWNLOADS`"]
    pub const DOWNLOADS: SystemDir = SystemDir {
        ord: 3i32
    };
    #[doc(alias = "SYSTEM_DIR_MOVIES")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_MOVIES`"]
    pub const MOVIES: SystemDir = SystemDir {
        ord: 4i32
    };
    #[doc(alias = "SYSTEM_DIR_MUSIC")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_MUSIC`"]
    pub const MUSIC: SystemDir = SystemDir {
        ord: 5i32
    };
    #[doc(alias = "SYSTEM_DIR_PICTURES")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_PICTURES`"]
    pub const PICTURES: SystemDir = SystemDir {
        ord: 6i32
    };
    #[doc(alias = "SYSTEM_DIR_RINGTONES")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_RINGTONES`"]
    pub const RINGTONES: SystemDir = SystemDir {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for SystemDir {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SystemDir") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SystemDir {
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
            Self::DESKTOP => "DESKTOP", Self::DCIM => "DCIM", Self::DOCUMENTS => "DOCUMENTS", Self::DOWNLOADS => "DOWNLOADS", Self::MOVIES => "MOVIES", Self::MUSIC => "MUSIC", Self::PICTURES => "PICTURES", Self::RINGTONES => "RINGTONES", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SystemDir::DESKTOP, SystemDir::DCIM, SystemDir::DOCUMENTS, SystemDir::DOWNLOADS, SystemDir::MOVIES, SystemDir::MUSIC, SystemDir::PICTURES, SystemDir::RINGTONES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SystemDir >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DESKTOP", "SYSTEM_DIR_DESKTOP", SystemDir::DESKTOP), crate::meta::inspect::EnumConstant::new("DCIM", "SYSTEM_DIR_DCIM", SystemDir::DCIM), crate::meta::inspect::EnumConstant::new("DOCUMENTS", "SYSTEM_DIR_DOCUMENTS", SystemDir::DOCUMENTS), crate::meta::inspect::EnumConstant::new("DOWNLOADS", "SYSTEM_DIR_DOWNLOADS", SystemDir::DOWNLOADS), crate::meta::inspect::EnumConstant::new("MOVIES", "SYSTEM_DIR_MOVIES", SystemDir::MOVIES), crate::meta::inspect::EnumConstant::new("MUSIC", "SYSTEM_DIR_MUSIC", SystemDir::MUSIC), crate::meta::inspect::EnumConstant::new("PICTURES", "SYSTEM_DIR_PICTURES", SystemDir::PICTURES), crate::meta::inspect::EnumConstant::new("RINGTONES", "SYSTEM_DIR_RINGTONES", SystemDir::RINGTONES)]
        }
    }
}
impl crate::meta::GodotConvert for SystemDir {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SystemDir {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SystemDir {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StdHandleType {
    ord: i32
}
impl StdHandleType {
    #[doc(alias = "STD_HANDLE_INVALID")]
    #[doc = "Godot enumerator name: `STD_HANDLE_INVALID`"]
    pub const INVALID: StdHandleType = StdHandleType {
        ord: 0i32
    };
    #[doc(alias = "STD_HANDLE_CONSOLE")]
    #[doc = "Godot enumerator name: `STD_HANDLE_CONSOLE`"]
    pub const CONSOLE: StdHandleType = StdHandleType {
        ord: 1i32
    };
    #[doc(alias = "STD_HANDLE_FILE")]
    #[doc = "Godot enumerator name: `STD_HANDLE_FILE`"]
    pub const FILE: StdHandleType = StdHandleType {
        ord: 2i32
    };
    #[doc(alias = "STD_HANDLE_PIPE")]
    #[doc = "Godot enumerator name: `STD_HANDLE_PIPE`"]
    pub const PIPE: StdHandleType = StdHandleType {
        ord: 3i32
    };
    #[doc(alias = "STD_HANDLE_UNKNOWN")]
    #[doc = "Godot enumerator name: `STD_HANDLE_UNKNOWN`"]
    pub const UNKNOWN: StdHandleType = StdHandleType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for StdHandleType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StdHandleType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StdHandleType {
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
            Self::INVALID => "INVALID", Self::CONSOLE => "CONSOLE", Self::FILE => "FILE", Self::PIPE => "PIPE", Self::UNKNOWN => "UNKNOWN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[StdHandleType::INVALID, StdHandleType::CONSOLE, StdHandleType::FILE, StdHandleType::PIPE, StdHandleType::UNKNOWN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < StdHandleType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INVALID", "STD_HANDLE_INVALID", StdHandleType::INVALID), crate::meta::inspect::EnumConstant::new("CONSOLE", "STD_HANDLE_CONSOLE", StdHandleType::CONSOLE), crate::meta::inspect::EnumConstant::new("FILE", "STD_HANDLE_FILE", StdHandleType::FILE), crate::meta::inspect::EnumConstant::new("PIPE", "STD_HANDLE_PIPE", StdHandleType::PIPE), crate::meta::inspect::EnumConstant::new("UNKNOWN", "STD_HANDLE_UNKNOWN", StdHandleType::UNKNOWN)]
        }
    }
}
impl crate::meta::GodotConvert for StdHandleType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for StdHandleType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StdHandleType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Os;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Os {
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