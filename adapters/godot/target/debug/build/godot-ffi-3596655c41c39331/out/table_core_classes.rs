type FetchFn = < crate::GDExtensionInterfaceClassdbGetMethodBind as crate::Inner > ::FnPtr;
pub struct ClassCoreMethodTable {
    function_pointers: Vec < crate::ClassMethodBind >,
}
impl ClassCoreMethodTable {
    pub const CLASS_COUNT: usize = 8usize;
    pub const METHOD_COUNT: usize = 239usize;
    #[doc = r" # Safety"]
    #[doc = r" - Must be called exactly once during library initialization."]
    #[doc = r" - All parameters (dependencies) must have been initialized and valid."]
    pub unsafe fn load(interface: &crate::GDExtensionInterface, string_names: &mut crate::StringCache,) -> Self {
        let fetch_fptr = interface.classdb_get_method_bind.expect("classdb_get_method_bind absent");
        let mut function_pointers = Vec::with_capacity(239usize);
        load_Engine_methods(&mut function_pointers, string_names, fetch_fptr);
        load_GDExtension_methods(&mut function_pointers, string_names, fetch_fptr);
        load_OS_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Object_methods(&mut function_pointers, string_names, fetch_fptr);
        load_ProjectSettings_methods(&mut function_pointers, string_names, fetch_fptr);
        load_RefCounted_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Resource_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Time_methods(&mut function_pointers, string_names, fetch_fptr);
        Self {
            function_pointers
        }
    }
    #[inline(always)]
    pub fn fptr_by_index(&self, index: usize) -> crate::ClassMethodBind {
        unsafe {
            * self.function_pointers.get_unchecked(index)
        }
    }
    #[inline(always)]
    pub fn os__has_feature(&self) -> crate::ClassMethodBind {
        self.fptr_by_index(118usize)
    }
    #[inline(always)]
    pub fn object__notification(&self) -> crate::ClassMethodBind {
        self.fptr_by_index(136usize)
    }
    #[inline(always)]
    pub fn object__to_string(&self) -> crate::ClassMethodBind {
        self.fptr_by_index(137usize)
    }
    #[inline(always)]
    pub fn object__set_script(&self) -> crate::ClassMethodBind {
        self.fptr_by_index(138usize)
    }
    #[inline(always)]
    pub fn object__get_script(&self) -> crate::ClassMethodBind {
        self.fptr_by_index(139usize)
    }
    #[inline(always)]
    pub fn object__connect(&self) -> crate::ClassMethodBind {
        self.fptr_by_index(159usize)
    }
    #[inline(always)]
    pub fn ref_counted__init_ref(&self) -> crate::ClassMethodBind {
        self.fptr_by_index(193usize)
    }
    #[inline(always)]
    pub fn ref_counted__reference(&self) -> crate::ClassMethodBind {
        self.fptr_by_index(194usize)
    }
    #[inline(always)]
    pub fn ref_counted__unreference(&self) -> crate::ClassMethodBind {
        self.fptr_by_index(195usize)
    }
}
fn load_Engine_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_Engine = string_names.fetch("Engine");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "set_physics_ticks_per_second", 1286410249i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_physics_ticks_per_second", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "set_max_physics_steps_per_frame", 1286410249i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_max_physics_steps_per_frame", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "set_physics_jitter_fix", 373806689i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_physics_jitter_fix", 1740695150i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_physics_interpolation_fraction", 1740695150i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "set_max_fps", 1286410249i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_max_fps", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "set_time_scale", 373806689i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_time_scale", 191475506i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_frames_drawn", 2455072627i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_frames_per_second", 1740695150i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_physics_frames", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_process_frames", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_main_loop", 1016888095i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_version_info", 3102165223i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_author_info", 3102165223i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_copyright_info", 3995934104i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_donor_info", 3102165223i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_license_info", 3102165223i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_license_text", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_architecture_name", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "is_in_physics_frame", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "has_singleton", 2619796661i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_singleton", 1371597918i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "register_singleton", 965313290i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "unregister_singleton", 3304788590i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_singleton_list", 1139954409i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "register_script_language", 1850254898i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "unregister_script_language", 1850254898i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_script_language_count", 2455072627i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_script_language", 2151255799i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "capture_script_backtraces", 873284517i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "is_editor_hint", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "is_embedded_in_editor", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "get_write_movie_path", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "set_print_to_stdout", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "is_printing_to_stdout", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "set_print_error_messages", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Engine), "Engine", "is_printing_error_messages", 36873697i64),);
    
}
fn load_GDExtension_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_GDExtension = string_names.fetch("GDExtension");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_GDExtension), "GDExtension", "is_library_open", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_GDExtension), "GDExtension", "get_minimum_library_initialization_level", 964858755i64),);
    
}
fn load_OS_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_OS = string_names.fetch("OS");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_entropy", 47165747i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_system_ca_certificates", 2841200299i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_connected_midi_inputs", 2981934095i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "open_midi_inputs", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "close_midi_inputs", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "alert", 1783970740i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "crash", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "set_low_processor_usage_mode", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "is_in_low_processor_usage_mode", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "set_low_processor_usage_mode_sleep_usec", 1286410249i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_low_processor_usage_mode_sleep_usec", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "set_delta_smoothing", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "is_delta_smoothing_enabled", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_processor_count", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_processor_name", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_system_fonts", 1139954409i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_system_font_path", 626580860i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_system_font_path_for_text", 197317981i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_executable_path", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "read_string_from_stdin", 723587915i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "read_buffer_from_stdin", 3249455752i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_stdin_type", 1704816237i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_stdout_type", 1704816237i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_stderr_type", 1704816237i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "execute", 1488299882i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "execute_with_pipe", 2851312030i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "create_process", 2903767230i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "create_instance", 1080601263i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "open_with_program", 2848259907i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "kill", 844576869i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "shell_open", 166001499i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "shell_show_in_file_manager", 3565188097i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "is_process_running", 1116898809i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_process_exit_code", 923996154i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_process_id", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "has_environment", 3927539163i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_environment", 3135753539i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "set_environment", 3605043004i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "unset_environment", 3089850668i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_name", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_distribution_name", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_version", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_version_alias", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_cmdline_args", 2981934095i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_cmdline_user_args", 2981934095i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_video_adapter_driver_info", 1139954409i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "set_restart_on_exit", 3331453935i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "is_restart_on_exit_set", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_restart_on_exit_arguments", 1139954409i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "delay_usec", 998575451i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "delay_msec", 998575451i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_locale", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_locale_language", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_model_name", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "is_userfs_persistent", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "is_stdout_verbose", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "is_debug_build", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_static_memory_usage", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_static_memory_peak_usage", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_memory_info", 3102165223i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "move_to_trash", 2113323047i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_user_data_dir", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_system_dir", 3073895123i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_config_dir", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_data_dir", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_cache_dir", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_temp_dir", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_unique_id", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_keycode_string", 2261993717i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "is_keycode_unicode", 1116898809i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "find_keycode_from_string", 1084858572i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "set_use_file_access_save_and_swap", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "set_thread_name", 166001499i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_thread_caller_id", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_main_thread_id", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "has_feature", 3927539163i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "is_sandboxed", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "request_permission", 2323990056i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "request_permissions", 2240911060i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "get_granted_permissions", 1139954409i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "revoke_granted_permissions", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "add_logger", 4261188958i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_OS), "OS", "remove_logger", 4261188958i64),);
    
}
fn load_Object_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_Object = string_names.fetch("Object");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "get_class", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "is_class", 3927539163i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "set", 3776071444i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "get", 2760726917i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "set_indexed", 3500910842i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "get_indexed", 4006125091i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "get_property_list", 3995934104i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "get_method_list", 3995934104i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "property_can_revert", 2619796661i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "property_get_revert", 2760726917i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "notification", 4023243586i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "to_string", 2841200299i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "set_script", 1114965689i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "get_script", 1214101251i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "set_meta", 3776071444i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "remove_meta", 3304788590i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "get_meta", 3990617847i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "has_meta", 2619796661i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "get_meta_list", 3995934104i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "add_user_signal", 85656714i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "has_user_signal", 2619796661i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "remove_user_signal", 3304788590i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "emit_signal", 4047867050i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "call", 3400424181i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "call_deferred", 3400424181i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "set_deferred", 3776071444i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "callv", 1260104456i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "has_method", 2619796661i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "get_method_argument_count", 2458036349i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "has_signal", 2619796661i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "get_signal_list", 3995934104i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "get_signal_connection_list", 3147814860i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "get_incoming_connections", 3995934104i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "connect", 1518946055i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "disconnect", 1874754934i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "is_connected", 768136979i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "has_connections", 2619796661i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "set_block_signals", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "is_blocking_signals", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "notify_property_list_changed", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "set_message_translation", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "can_translate_messages", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "tr", 1195764410i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "tr_n", 162698058i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "get_translation_domain", 2002593661i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "set_translation_domain", 3304788590i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "is_queued_for_deletion", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Object), "Object", "cancel_free", 3218959716i64),);
    
}
fn load_ProjectSettings_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_ProjectSettings = string_names.fetch("ProjectSettings");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "has_setting", 3927539163i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "set_setting", 402577236i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "get_setting", 223050753i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "get_setting_with_override", 2760726917i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "get_global_class_list", 2915620761i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "get_setting_with_override_and_custom_features", 2434817427i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "set_order", 2956805083i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "get_order", 1321353865i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "set_initial_value", 402577236i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "set_as_basic", 2678287736i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "set_as_internal", 2678287736i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "add_property_info", 4155329257i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "set_restart_if_changed", 2678287736i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "clear", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "localize_path", 3135753539i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "globalize_path", 3135753539i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "save", 166280745i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "load_resource_pack", 708980503i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ProjectSettings), "ProjectSettings", "save_custom", 166001499i64),);
    
}
fn load_RefCounted_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_RefCounted = string_names.fetch("RefCounted");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_RefCounted), "RefCounted", "init_ref", 2240911060i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_RefCounted), "RefCounted", "reference", 2240911060i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_RefCounted), "RefCounted", "unreference", 2240911060i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_RefCounted), "RefCounted", "get_reference_count", 3905245786i64),);
    
}
fn load_Resource_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_Resource = string_names.fetch("Resource");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "set_path", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "take_over_path", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "get_path", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "set_path_cache", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "set_name", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "get_name", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "get_rid", 2944877500i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "set_local_to_scene", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "is_local_to_scene", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "get_local_scene", 3160264692i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "setup_local_to_scene", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "reset_state", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "set_id_for_path", 3186203200i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "get_id_for_path", 3135753539i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "is_built_in", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "generate_scene_unique_id", 2841200299i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "set_scene_unique_id", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "get_scene_unique_id", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "emit_changed", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "duplicate", 482882304i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Resource), "Resource", "duplicate_deep", 905779109i64),);
    
}
fn load_Time_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_Time = string_names.fetch("Time");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_datetime_dict_from_unix_time", 3485342025i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_date_dict_from_unix_time", 3485342025i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_time_dict_from_unix_time", 3485342025i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_datetime_string_from_unix_time", 2311239925i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_date_string_from_unix_time", 844755477i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_time_string_from_unix_time", 844755477i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_datetime_dict_from_datetime_string", 3253569256i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_datetime_string_from_datetime_dict", 1898123706i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_unix_time_from_datetime_dict", 3021115443i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_unix_time_from_datetime_string", 1321353865i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_offset_string_from_offset_minutes", 844755477i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_datetime_dict_from_system", 205769976i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_date_dict_from_system", 205769976i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_time_dict_from_system", 205769976i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_datetime_string_from_system", 1136425492i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_date_string_from_system", 1162154673i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_time_string_from_system", 1162154673i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_time_zone_from_system", 3102165223i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_unix_time_from_system", 1740695150i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_ticks_msec", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_Time), "Time", "get_ticks_usec", 3905245786i64),);
    
}