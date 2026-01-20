type FetchFn = < crate::GDExtensionInterfaceVariantGetPtrBuiltinMethod as crate::Inner > ::FnPtr;
pub struct BuiltinMethodTable {
    function_pointers: Vec < crate::BuiltinMethodBind >,
}
impl BuiltinMethodTable {
    pub const CLASS_COUNT: usize = 34usize;
    pub const METHOD_COUNT: usize = 998usize;
    #[doc = r" # Safety"]
    #[doc = r" - Must be called exactly once during library initialization."]
    #[doc = r" - All parameters (dependencies) must have been initialized and valid."]
    pub unsafe fn load(interface: &crate::GDExtensionInterface, string_names: &mut crate::StringCache,) -> Self {
        let fetch_fptr = interface.variant_get_ptr_builtin_method.expect("variant_get_ptr_builtin_method absent");
        let mut function_pointers = Vec::with_capacity(998usize);
        load_String_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Vector2_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Vector2i_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Rect2_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Rect2i_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Vector3_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Vector3i_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Transform2D_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Vector4_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Vector4i_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Plane_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Quaternion_methods(&mut function_pointers, string_names, fetch_fptr);
        load_AABB_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Basis_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Transform3D_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Projection_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Color_methods(&mut function_pointers, string_names, fetch_fptr);
        load_StringName_methods(&mut function_pointers, string_names, fetch_fptr);
        load_NodePath_methods(&mut function_pointers, string_names, fetch_fptr);
        load_RID_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Callable_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Signal_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Dictionary_methods(&mut function_pointers, string_names, fetch_fptr);
        load_Array_methods(&mut function_pointers, string_names, fetch_fptr);
        load_PackedByteArray_methods(&mut function_pointers, string_names, fetch_fptr);
        load_PackedInt32Array_methods(&mut function_pointers, string_names, fetch_fptr);
        load_PackedInt64Array_methods(&mut function_pointers, string_names, fetch_fptr);
        load_PackedFloat32Array_methods(&mut function_pointers, string_names, fetch_fptr);
        load_PackedFloat64Array_methods(&mut function_pointers, string_names, fetch_fptr);
        load_PackedStringArray_methods(&mut function_pointers, string_names, fetch_fptr);
        load_PackedVector2Array_methods(&mut function_pointers, string_names, fetch_fptr);
        load_PackedVector3Array_methods(&mut function_pointers, string_names, fetch_fptr);
        load_PackedColorArray_methods(&mut function_pointers, string_names, fetch_fptr);
        load_PackedVector4Array_methods(&mut function_pointers, string_names, fetch_fptr);
        Self {
            function_pointers
        }
    }
    #[inline(always)]
    pub fn fptr_by_index(&self, index: usize) -> crate::BuiltinMethodBind {
        unsafe {
            * self.function_pointers.get_unchecked(index)
        }
    }
}
fn load_String_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 0usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "casecmp_to", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 1usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "nocasecmp_to", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 2usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "naturalcasecmp_to", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 3usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "naturalnocasecmp_to", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 4usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "filecasecmp_to", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 5usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "filenocasecmp_to", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 6usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "length", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 7usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "substr", 787537301i64)
    },
    );
    function_pointers.push({
        let _ = 8usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "get_slice", 3535100402i64)
    },
    );
    function_pointers.push({
        let _ = 9usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "get_slicec", 787537301i64)
    },
    );
    function_pointers.push({
        let _ = 10usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "get_slice_count", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 11usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "find", 1760645412i64)
    },
    );
    function_pointers.push({
        let _ = 12usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "findn", 1760645412i64)
    },
    );
    function_pointers.push({
        let _ = 13usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "count", 2343087891i64)
    },
    );
    function_pointers.push({
        let _ = 14usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "countn", 2343087891i64)
    },
    );
    function_pointers.push({
        let _ = 15usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "rfind", 1760645412i64)
    },
    );
    function_pointers.push({
        let _ = 16usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "rfindn", 1760645412i64)
    },
    );
    function_pointers.push({
        let _ = 17usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "match", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 18usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "matchn", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 19usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "begins_with", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 20usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "ends_with", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 21usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_subsequence_of", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 22usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_subsequence_ofn", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 23usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "bigrams", 747180633i64)
    },
    );
    function_pointers.push({
        let _ = 24usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "similarity", 2697460964i64)
    },
    );
    function_pointers.push({
        let _ = 25usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "format", 3212199029i64)
    },
    );
    function_pointers.push({
        let _ = 26usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "replace", 1340436205i64)
    },
    );
    function_pointers.push({
        let _ = 27usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "replacen", 1340436205i64)
    },
    );
    function_pointers.push({
        let _ = 28usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "replace_char", 787537301i64)
    },
    );
    function_pointers.push({
        let _ = 29usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "replace_chars", 3535100402i64)
    },
    );
    function_pointers.push({
        let _ = 30usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "remove_char", 2162347432i64)
    },
    );
    function_pointers.push({
        let _ = 31usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "remove_chars", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 32usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "repeat", 2162347432i64)
    },
    );
    function_pointers.push({
        let _ = 33usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "reverse", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 34usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "insert", 248737229i64)
    },
    );
    function_pointers.push({
        let _ = 35usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "erase", 787537301i64)
    },
    );
    function_pointers.push({
        let _ = 36usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "capitalize", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 37usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_camel_case", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 38usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_pascal_case", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 39usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_snake_case", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 40usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_kebab_case", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 41usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "split", 1252735785i64)
    },
    );
    function_pointers.push({
        let _ = 42usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "rsplit", 1252735785i64)
    },
    );
    function_pointers.push({
        let _ = 43usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "split_floats", 2092079095i64)
    },
    );
    function_pointers.push({
        let _ = 44usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "join", 3595973238i64)
    },
    );
    function_pointers.push({
        let _ = 45usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_upper", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 46usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_lower", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 47usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "left", 2162347432i64)
    },
    );
    function_pointers.push({
        let _ = 48usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "right", 2162347432i64)
    },
    );
    function_pointers.push({
        let _ = 49usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "strip_edges", 907855311i64)
    },
    );
    function_pointers.push({
        let _ = 50usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "strip_escapes", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 51usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "lstrip", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 52usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "rstrip", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 53usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "get_extension", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 54usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "get_basename", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 55usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "path_join", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 56usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "unicode_at", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 57usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "indent", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 58usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "dedent", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 59usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "hash", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 60usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "md5_text", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 61usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "sha1_text", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 62usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "sha256_text", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 63usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "md5_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 64usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "sha1_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 65usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "sha256_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 66usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 67usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "contains", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 68usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "containsn", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 69usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_absolute_path", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 70usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_relative_path", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 71usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "simplify_path", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 72usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "get_base_dir", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 73usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "get_file", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 74usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "xml_escape", 3429816538i64)
    },
    );
    function_pointers.push({
        let _ = 75usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "xml_unescape", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 76usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "uri_encode", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 77usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "uri_decode", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 78usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "uri_file_decode", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 79usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "c_escape", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 80usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "c_unescape", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 81usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "json_escape", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 82usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "validate_node_name", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 83usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "validate_filename", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 84usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_valid_ascii_identifier", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 85usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_valid_unicode_identifier", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 86usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_valid_identifier", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 87usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_valid_int", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 88usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_valid_float", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 89usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_valid_hex_number", 593672999i64)
    },
    );
    function_pointers.push({
        let _ = 90usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_valid_html_color", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 91usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_valid_ip_address", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 92usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "is_valid_filename", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 93usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_int", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 94usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_float", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 95usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "hex_to_int", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 96usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "bin_to_int", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 97usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "lpad", 248737229i64)
    },
    );
    function_pointers.push({
        let _ = 98usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "rpad", 248737229i64)
    },
    );
    function_pointers.push({
        let _ = 99usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "pad_decimals", 2162347432i64)
    },
    );
    function_pointers.push({
        let _ = 100usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "pad_zeros", 2162347432i64)
    },
    );
    function_pointers.push({
        let _ = 101usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "trim_prefix", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 102usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "trim_suffix", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 103usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_ascii_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 104usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_utf8_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 105usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_utf16_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 106usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_utf32_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 107usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_wchar_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 108usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "to_multibyte_char_buffer", 3055765187i64)
    },
    );
    function_pointers.push({
        let _ = 109usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "hex_decode", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 110usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "num_scientific", 2710373411i64)
    },
    );
    function_pointers.push({
        let _ = 111usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "num", 1555901022i64)
    },
    );
    function_pointers.push({
        let _ = 112usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "num_int64", 2111271071i64)
    },
    );
    function_pointers.push({
        let _ = 113usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "num_uint64", 2111271071i64)
    },
    );
    function_pointers.push({
        let _ = 114usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "chr", 897497541i64)
    },
    );
    function_pointers.push({
        let _ = 115usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING, "String", "humanize_size", 897497541i64)
    },
    );
    
}
fn load_Vector2_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 116usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "angle", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 117usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "angle_to", 3819070308i64)
    },
    );
    function_pointers.push({
        let _ = 118usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "angle_to_point", 3819070308i64)
    },
    );
    function_pointers.push({
        let _ = 119usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "direction_to", 2026743667i64)
    },
    );
    function_pointers.push({
        let _ = 120usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "distance_to", 3819070308i64)
    },
    );
    function_pointers.push({
        let _ = 121usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "distance_squared_to", 3819070308i64)
    },
    );
    function_pointers.push({
        let _ = 122usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "length", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 123usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "length_squared", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 124usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "limit_length", 2544004089i64)
    },
    );
    function_pointers.push({
        let _ = 125usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "normalized", 2428350749i64)
    },
    );
    function_pointers.push({
        let _ = 126usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "is_normalized", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 127usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "is_equal_approx", 3190634762i64)
    },
    );
    function_pointers.push({
        let _ = 128usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "is_zero_approx", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 129usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "is_finite", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 130usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "posmod", 2544004089i64)
    },
    );
    function_pointers.push({
        let _ = 131usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "posmodv", 2026743667i64)
    },
    );
    function_pointers.push({
        let _ = 132usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "project", 2026743667i64)
    },
    );
    function_pointers.push({
        let _ = 133usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "lerp", 4250033116i64)
    },
    );
    function_pointers.push({
        let _ = 134usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "slerp", 4250033116i64)
    },
    );
    function_pointers.push({
        let _ = 135usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "cubic_interpolate", 193522989i64)
    },
    );
    function_pointers.push({
        let _ = 136usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "cubic_interpolate_in_time", 1957055074i64)
    },
    );
    function_pointers.push({
        let _ = 137usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "bezier_interpolate", 193522989i64)
    },
    );
    function_pointers.push({
        let _ = 138usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "bezier_derivative", 193522989i64)
    },
    );
    function_pointers.push({
        let _ = 139usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "max_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 140usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "min_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 141usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "move_toward", 4250033116i64)
    },
    );
    function_pointers.push({
        let _ = 142usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "rotated", 2544004089i64)
    },
    );
    function_pointers.push({
        let _ = 143usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "orthogonal", 2428350749i64)
    },
    );
    function_pointers.push({
        let _ = 144usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "floor", 2428350749i64)
    },
    );
    function_pointers.push({
        let _ = 145usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "ceil", 2428350749i64)
    },
    );
    function_pointers.push({
        let _ = 146usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "round", 2428350749i64)
    },
    );
    function_pointers.push({
        let _ = 147usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "aspect", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 148usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "dot", 3819070308i64)
    },
    );
    function_pointers.push({
        let _ = 149usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "slide", 2026743667i64)
    },
    );
    function_pointers.push({
        let _ = 150usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "bounce", 2026743667i64)
    },
    );
    function_pointers.push({
        let _ = 151usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "reflect", 2026743667i64)
    },
    );
    function_pointers.push({
        let _ = 152usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "cross", 3819070308i64)
    },
    );
    function_pointers.push({
        let _ = 153usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "abs", 2428350749i64)
    },
    );
    function_pointers.push({
        let _ = 154usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "sign", 2428350749i64)
    },
    );
    function_pointers.push({
        let _ = 155usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "clamp", 318031021i64)
    },
    );
    function_pointers.push({
        let _ = 156usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "clampf", 3464402636i64)
    },
    );
    function_pointers.push({
        let _ = 157usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "snapped", 2026743667i64)
    },
    );
    function_pointers.push({
        let _ = 158usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "snappedf", 2544004089i64)
    },
    );
    function_pointers.push({
        let _ = 159usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "min", 2026743667i64)
    },
    );
    function_pointers.push({
        let _ = 160usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "minf", 2544004089i64)
    },
    );
    function_pointers.push({
        let _ = 161usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "max", 2026743667i64)
    },
    );
    function_pointers.push({
        let _ = 162usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "maxf", 2544004089i64)
    },
    );
    function_pointers.push({
        let _ = 163usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, "Vector2", "from_angle", 889263119i64)
    },
    );
    
}
fn load_Vector2i_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 164usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "aspect", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 165usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "max_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 166usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "min_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 167usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "distance_to", 707501214i64)
    },
    );
    function_pointers.push({
        let _ = 168usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "distance_squared_to", 1130029528i64)
    },
    );
    function_pointers.push({
        let _ = 169usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "length", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 170usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "length_squared", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 171usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "sign", 3444277866i64)
    },
    );
    function_pointers.push({
        let _ = 172usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "abs", 3444277866i64)
    },
    );
    function_pointers.push({
        let _ = 173usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "clamp", 186568249i64)
    },
    );
    function_pointers.push({
        let _ = 174usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "clampi", 3686769569i64)
    },
    );
    function_pointers.push({
        let _ = 175usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "snapped", 1735278196i64)
    },
    );
    function_pointers.push({
        let _ = 176usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "snappedi", 2161988953i64)
    },
    );
    function_pointers.push({
        let _ = 177usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "min", 1735278196i64)
    },
    );
    function_pointers.push({
        let _ = 178usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "mini", 2161988953i64)
    },
    );
    function_pointers.push({
        let _ = 179usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "max", 1735278196i64)
    },
    );
    function_pointers.push({
        let _ = 180usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, "Vector2i", "maxi", 2161988953i64)
    },
    );
    
}
fn load_Rect2_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 181usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "get_center", 2428350749i64)
    },
    );
    function_pointers.push({
        let _ = 182usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "get_area", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 183usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "has_area", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 184usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "has_point", 3190634762i64)
    },
    );
    function_pointers.push({
        let _ = 185usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "is_equal_approx", 1908192260i64)
    },
    );
    function_pointers.push({
        let _ = 186usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "is_finite", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 187usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "intersects", 819294880i64)
    },
    );
    function_pointers.push({
        let _ = 188usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "encloses", 1908192260i64)
    },
    );
    function_pointers.push({
        let _ = 189usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "intersection", 2282977743i64)
    },
    );
    function_pointers.push({
        let _ = 190usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "merge", 2282977743i64)
    },
    );
    function_pointers.push({
        let _ = 191usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "expand", 293272265i64)
    },
    );
    function_pointers.push({
        let _ = 192usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "get_support", 2026743667i64)
    },
    );
    function_pointers.push({
        let _ = 193usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "grow", 39664498i64)
    },
    );
    function_pointers.push({
        let _ = 194usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "grow_side", 4177736158i64)
    },
    );
    function_pointers.push({
        let _ = 195usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "grow_individual", 3203390369i64)
    },
    );
    function_pointers.push({
        let _ = 196usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2, "Rect2", "abs", 3107653634i64)
    },
    );
    
}
fn load_Rect2i_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 197usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, "Rect2i", "get_center", 3444277866i64)
    },
    );
    function_pointers.push({
        let _ = 198usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, "Rect2i", "get_area", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 199usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, "Rect2i", "has_area", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 200usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, "Rect2i", "has_point", 328189994i64)
    },
    );
    function_pointers.push({
        let _ = 201usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, "Rect2i", "intersects", 3434691493i64)
    },
    );
    function_pointers.push({
        let _ = 202usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, "Rect2i", "encloses", 3434691493i64)
    },
    );
    function_pointers.push({
        let _ = 203usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, "Rect2i", "intersection", 717431873i64)
    },
    );
    function_pointers.push({
        let _ = 204usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, "Rect2i", "merge", 717431873i64)
    },
    );
    function_pointers.push({
        let _ = 205usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, "Rect2i", "expand", 1355196872i64)
    },
    );
    function_pointers.push({
        let _ = 206usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, "Rect2i", "grow", 1578070074i64)
    },
    );
    function_pointers.push({
        let _ = 207usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, "Rect2i", "grow_side", 3191154199i64)
    },
    );
    function_pointers.push({
        let _ = 208usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, "Rect2i", "grow_individual", 1893743416i64)
    },
    );
    function_pointers.push({
        let _ = 209usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, "Rect2i", "abs", 1469025700i64)
    },
    );
    
}
fn load_Vector3_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 210usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "min_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 211usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "max_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 212usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "angle_to", 1047977935i64)
    },
    );
    function_pointers.push({
        let _ = 213usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "signed_angle_to", 2781412522i64)
    },
    );
    function_pointers.push({
        let _ = 214usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "direction_to", 2923479887i64)
    },
    );
    function_pointers.push({
        let _ = 215usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "distance_to", 1047977935i64)
    },
    );
    function_pointers.push({
        let _ = 216usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "distance_squared_to", 1047977935i64)
    },
    );
    function_pointers.push({
        let _ = 217usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "length", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 218usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "length_squared", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 219usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "limit_length", 514930144i64)
    },
    );
    function_pointers.push({
        let _ = 220usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "normalized", 1776574132i64)
    },
    );
    function_pointers.push({
        let _ = 221usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "is_normalized", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 222usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "is_equal_approx", 1749054343i64)
    },
    );
    function_pointers.push({
        let _ = 223usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "is_zero_approx", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 224usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "is_finite", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 225usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "inverse", 1776574132i64)
    },
    );
    function_pointers.push({
        let _ = 226usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "clamp", 4145107892i64)
    },
    );
    function_pointers.push({
        let _ = 227usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "clampf", 2329594628i64)
    },
    );
    function_pointers.push({
        let _ = 228usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "snapped", 2923479887i64)
    },
    );
    function_pointers.push({
        let _ = 229usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "snappedf", 514930144i64)
    },
    );
    function_pointers.push({
        let _ = 230usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "rotated", 1682608829i64)
    },
    );
    function_pointers.push({
        let _ = 231usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "lerp", 1682608829i64)
    },
    );
    function_pointers.push({
        let _ = 232usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "slerp", 1682608829i64)
    },
    );
    function_pointers.push({
        let _ = 233usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "cubic_interpolate", 2597922253i64)
    },
    );
    function_pointers.push({
        let _ = 234usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "cubic_interpolate_in_time", 3256682901i64)
    },
    );
    function_pointers.push({
        let _ = 235usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "bezier_interpolate", 2597922253i64)
    },
    );
    function_pointers.push({
        let _ = 236usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "bezier_derivative", 2597922253i64)
    },
    );
    function_pointers.push({
        let _ = 237usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "move_toward", 1682608829i64)
    },
    );
    function_pointers.push({
        let _ = 238usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "dot", 1047977935i64)
    },
    );
    function_pointers.push({
        let _ = 239usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "cross", 2923479887i64)
    },
    );
    function_pointers.push({
        let _ = 240usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "outer", 3934786792i64)
    },
    );
    function_pointers.push({
        let _ = 241usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "abs", 1776574132i64)
    },
    );
    function_pointers.push({
        let _ = 242usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "floor", 1776574132i64)
    },
    );
    function_pointers.push({
        let _ = 243usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "ceil", 1776574132i64)
    },
    );
    function_pointers.push({
        let _ = 244usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "round", 1776574132i64)
    },
    );
    function_pointers.push({
        let _ = 245usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "posmod", 514930144i64)
    },
    );
    function_pointers.push({
        let _ = 246usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "posmodv", 2923479887i64)
    },
    );
    function_pointers.push({
        let _ = 247usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "project", 2923479887i64)
    },
    );
    function_pointers.push({
        let _ = 248usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "slide", 2923479887i64)
    },
    );
    function_pointers.push({
        let _ = 249usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "bounce", 2923479887i64)
    },
    );
    function_pointers.push({
        let _ = 250usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "reflect", 2923479887i64)
    },
    );
    function_pointers.push({
        let _ = 251usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "sign", 1776574132i64)
    },
    );
    function_pointers.push({
        let _ = 252usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "octahedron_encode", 2428350749i64)
    },
    );
    function_pointers.push({
        let _ = 253usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "min", 2923479887i64)
    },
    );
    function_pointers.push({
        let _ = 254usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "minf", 514930144i64)
    },
    );
    function_pointers.push({
        let _ = 255usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "max", 2923479887i64)
    },
    );
    function_pointers.push({
        let _ = 256usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "maxf", 514930144i64)
    },
    );
    function_pointers.push({
        let _ = 257usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, "Vector3", "octahedron_decode", 3991820552i64)
    },
    );
    
}
fn load_Vector3i_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 258usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "min_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 259usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "max_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 260usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "distance_to", 1975170430i64)
    },
    );
    function_pointers.push({
        let _ = 261usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "distance_squared_to", 2947717320i64)
    },
    );
    function_pointers.push({
        let _ = 262usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "length", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 263usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "length_squared", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 264usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "sign", 3729604559i64)
    },
    );
    function_pointers.push({
        let _ = 265usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "abs", 3729604559i64)
    },
    );
    function_pointers.push({
        let _ = 266usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "clamp", 1086892323i64)
    },
    );
    function_pointers.push({
        let _ = 267usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "clampi", 1077216921i64)
    },
    );
    function_pointers.push({
        let _ = 268usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "snapped", 1989319750i64)
    },
    );
    function_pointers.push({
        let _ = 269usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "snappedi", 2377625641i64)
    },
    );
    function_pointers.push({
        let _ = 270usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "min", 1989319750i64)
    },
    );
    function_pointers.push({
        let _ = 271usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "mini", 2377625641i64)
    },
    );
    function_pointers.push({
        let _ = 272usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "max", 1989319750i64)
    },
    );
    function_pointers.push({
        let _ = 273usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, "Vector3i", "maxi", 2377625641i64)
    },
    );
    
}
fn load_Transform2D_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 274usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "inverse", 1420440541i64)
    },
    );
    function_pointers.push({
        let _ = 275usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "affine_inverse", 1420440541i64)
    },
    );
    function_pointers.push({
        let _ = 276usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "get_rotation", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 277usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "get_origin", 2428350749i64)
    },
    );
    function_pointers.push({
        let _ = 278usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "get_scale", 2428350749i64)
    },
    );
    function_pointers.push({
        let _ = 279usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "get_skew", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 280usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "orthonormalized", 1420440541i64)
    },
    );
    function_pointers.push({
        let _ = 281usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "rotated", 729597514i64)
    },
    );
    function_pointers.push({
        let _ = 282usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "rotated_local", 729597514i64)
    },
    );
    function_pointers.push({
        let _ = 283usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "scaled", 1446323263i64)
    },
    );
    function_pointers.push({
        let _ = 284usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "scaled_local", 1446323263i64)
    },
    );
    function_pointers.push({
        let _ = 285usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "translated", 1446323263i64)
    },
    );
    function_pointers.push({
        let _ = 286usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "translated_local", 1446323263i64)
    },
    );
    function_pointers.push({
        let _ = 287usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "determinant", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 288usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "basis_xform", 2026743667i64)
    },
    );
    function_pointers.push({
        let _ = 289usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "basis_xform_inv", 2026743667i64)
    },
    );
    function_pointers.push({
        let _ = 290usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "interpolate_with", 359399686i64)
    },
    );
    function_pointers.push({
        let _ = 291usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "is_conformal", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 292usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "is_equal_approx", 3837431929i64)
    },
    );
    function_pointers.push({
        let _ = 293usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "is_finite", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 294usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, "Transform2D", "looking_at", 1446323263i64)
    },
    );
    
}
fn load_Vector4_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 295usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "min_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 296usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "max_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 297usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "length", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 298usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "length_squared", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 299usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "abs", 80860099i64)
    },
    );
    function_pointers.push({
        let _ = 300usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "sign", 80860099i64)
    },
    );
    function_pointers.push({
        let _ = 301usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "floor", 80860099i64)
    },
    );
    function_pointers.push({
        let _ = 302usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "ceil", 80860099i64)
    },
    );
    function_pointers.push({
        let _ = 303usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "round", 80860099i64)
    },
    );
    function_pointers.push({
        let _ = 304usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "lerp", 2329757942i64)
    },
    );
    function_pointers.push({
        let _ = 305usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "cubic_interpolate", 726768410i64)
    },
    );
    function_pointers.push({
        let _ = 306usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "cubic_interpolate_in_time", 681631873i64)
    },
    );
    function_pointers.push({
        let _ = 307usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "posmod", 3129671720i64)
    },
    );
    function_pointers.push({
        let _ = 308usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "posmodv", 2031281584i64)
    },
    );
    function_pointers.push({
        let _ = 309usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "snapped", 2031281584i64)
    },
    );
    function_pointers.push({
        let _ = 310usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "snappedf", 3129671720i64)
    },
    );
    function_pointers.push({
        let _ = 311usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "clamp", 823915692i64)
    },
    );
    function_pointers.push({
        let _ = 312usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "clampf", 4072091586i64)
    },
    );
    function_pointers.push({
        let _ = 313usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "normalized", 80860099i64)
    },
    );
    function_pointers.push({
        let _ = 314usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "is_normalized", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 315usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "direction_to", 2031281584i64)
    },
    );
    function_pointers.push({
        let _ = 316usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "distance_to", 3770801042i64)
    },
    );
    function_pointers.push({
        let _ = 317usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "distance_squared_to", 3770801042i64)
    },
    );
    function_pointers.push({
        let _ = 318usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "dot", 3770801042i64)
    },
    );
    function_pointers.push({
        let _ = 319usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "inverse", 80860099i64)
    },
    );
    function_pointers.push({
        let _ = 320usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "is_equal_approx", 88913544i64)
    },
    );
    function_pointers.push({
        let _ = 321usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "is_zero_approx", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 322usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "is_finite", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 323usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "min", 2031281584i64)
    },
    );
    function_pointers.push({
        let _ = 324usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "minf", 3129671720i64)
    },
    );
    function_pointers.push({
        let _ = 325usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "max", 2031281584i64)
    },
    );
    function_pointers.push({
        let _ = 326usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, "Vector4", "maxf", 3129671720i64)
    },
    );
    
}
fn load_Vector4i_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 327usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "min_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 328usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "max_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 329usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "length", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 330usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "length_squared", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 331usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "sign", 4134919947i64)
    },
    );
    function_pointers.push({
        let _ = 332usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "abs", 4134919947i64)
    },
    );
    function_pointers.push({
        let _ = 333usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "clamp", 3046490913i64)
    },
    );
    function_pointers.push({
        let _ = 334usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "clampi", 2994578256i64)
    },
    );
    function_pointers.push({
        let _ = 335usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "snapped", 1181693102i64)
    },
    );
    function_pointers.push({
        let _ = 336usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "snappedi", 1476494415i64)
    },
    );
    function_pointers.push({
        let _ = 337usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "min", 1181693102i64)
    },
    );
    function_pointers.push({
        let _ = 338usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "mini", 1476494415i64)
    },
    );
    function_pointers.push({
        let _ = 339usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "max", 1181693102i64)
    },
    );
    function_pointers.push({
        let _ = 340usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "maxi", 1476494415i64)
    },
    );
    function_pointers.push({
        let _ = 341usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "distance_to", 3446086573i64)
    },
    );
    function_pointers.push({
        let _ = 342usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, "Vector4i", "distance_squared_to", 346708794i64)
    },
    );
    
}
fn load_Plane_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 343usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PLANE, "Plane", "normalized", 1051796340i64)
    },
    );
    function_pointers.push({
        let _ = 344usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PLANE, "Plane", "get_center", 1776574132i64)
    },
    );
    function_pointers.push({
        let _ = 345usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PLANE, "Plane", "is_equal_approx", 1150170233i64)
    },
    );
    function_pointers.push({
        let _ = 346usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PLANE, "Plane", "is_finite", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 347usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PLANE, "Plane", "is_point_over", 1749054343i64)
    },
    );
    function_pointers.push({
        let _ = 348usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PLANE, "Plane", "distance_to", 1047977935i64)
    },
    );
    function_pointers.push({
        let _ = 349usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PLANE, "Plane", "has_point", 1258189072i64)
    },
    );
    function_pointers.push({
        let _ = 350usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PLANE, "Plane", "project", 2923479887i64)
    },
    );
    function_pointers.push({
        let _ = 351usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PLANE, "Plane", "intersect_3", 2012052692i64)
    },
    );
    function_pointers.push({
        let _ = 352usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PLANE, "Plane", "intersects_ray", 2048133369i64)
    },
    );
    function_pointers.push({
        let _ = 353usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PLANE, "Plane", "intersects_segment", 2048133369i64)
    },
    );
    
}
fn load_Quaternion_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 354usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "length", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 355usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "length_squared", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 356usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "normalized", 4274879941i64)
    },
    );
    function_pointers.push({
        let _ = 357usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "is_normalized", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 358usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "is_equal_approx", 1682156903i64)
    },
    );
    function_pointers.push({
        let _ = 359usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "is_finite", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 360usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "inverse", 4274879941i64)
    },
    );
    function_pointers.push({
        let _ = 361usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "log", 4274879941i64)
    },
    );
    function_pointers.push({
        let _ = 362usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "exp", 4274879941i64)
    },
    );
    function_pointers.push({
        let _ = 363usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "angle_to", 3244682419i64)
    },
    );
    function_pointers.push({
        let _ = 364usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "dot", 3244682419i64)
    },
    );
    function_pointers.push({
        let _ = 365usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "slerp", 1773590316i64)
    },
    );
    function_pointers.push({
        let _ = 366usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "slerpni", 1773590316i64)
    },
    );
    function_pointers.push({
        let _ = 367usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "spherical_cubic_interpolate", 2150967576i64)
    },
    );
    function_pointers.push({
        let _ = 368usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "spherical_cubic_interpolate_in_time", 1436023539i64)
    },
    );
    function_pointers.push({
        let _ = 369usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "get_euler", 1394941017i64)
    },
    );
    function_pointers.push({
        let _ = 370usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "from_euler", 4053467903i64)
    },
    );
    function_pointers.push({
        let _ = 371usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "get_axis", 1776574132i64)
    },
    );
    function_pointers.push({
        let _ = 372usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, "Quaternion", "get_angle", 466405837i64)
    },
    );
    
}
fn load_AABB_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 373usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "abs", 1576868580i64)
    },
    );
    function_pointers.push({
        let _ = 374usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "get_center", 1776574132i64)
    },
    );
    function_pointers.push({
        let _ = 375usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "get_volume", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 376usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "has_volume", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 377usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "has_surface", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 378usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "has_point", 1749054343i64)
    },
    );
    function_pointers.push({
        let _ = 379usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "is_equal_approx", 299946684i64)
    },
    );
    function_pointers.push({
        let _ = 380usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "is_finite", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 381usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "intersects", 299946684i64)
    },
    );
    function_pointers.push({
        let _ = 382usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "encloses", 299946684i64)
    },
    );
    function_pointers.push({
        let _ = 383usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "intersects_plane", 1150170233i64)
    },
    );
    function_pointers.push({
        let _ = 384usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "intersection", 1271470306i64)
    },
    );
    function_pointers.push({
        let _ = 385usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "merge", 1271470306i64)
    },
    );
    function_pointers.push({
        let _ = 386usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "expand", 2851643018i64)
    },
    );
    function_pointers.push({
        let _ = 387usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "grow", 239217291i64)
    },
    );
    function_pointers.push({
        let _ = 388usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "get_support", 2923479887i64)
    },
    );
    function_pointers.push({
        let _ = 389usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "get_longest_axis", 1776574132i64)
    },
    );
    function_pointers.push({
        let _ = 390usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "get_longest_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 391usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "get_longest_axis_size", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 392usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "get_shortest_axis", 1776574132i64)
    },
    );
    function_pointers.push({
        let _ = 393usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "get_shortest_axis_index", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 394usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "get_shortest_axis_size", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 395usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "get_endpoint", 1394941017i64)
    },
    );
    function_pointers.push({
        let _ = 396usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "intersects_segment", 2048133369i64)
    },
    );
    function_pointers.push({
        let _ = 397usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_AABB, "AABB", "intersects_ray", 2048133369i64)
    },
    );
    
}
fn load_Basis_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 398usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "inverse", 594669093i64)
    },
    );
    function_pointers.push({
        let _ = 399usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "transposed", 594669093i64)
    },
    );
    function_pointers.push({
        let _ = 400usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "orthonormalized", 594669093i64)
    },
    );
    function_pointers.push({
        let _ = 401usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "determinant", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 402usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "rotated", 1998708965i64)
    },
    );
    function_pointers.push({
        let _ = 403usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "scaled", 3934786792i64)
    },
    );
    function_pointers.push({
        let _ = 404usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "scaled_local", 3934786792i64)
    },
    );
    function_pointers.push({
        let _ = 405usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "get_scale", 1776574132i64)
    },
    );
    function_pointers.push({
        let _ = 406usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "get_euler", 1394941017i64)
    },
    );
    function_pointers.push({
        let _ = 407usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "tdotx", 1047977935i64)
    },
    );
    function_pointers.push({
        let _ = 408usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "tdoty", 1047977935i64)
    },
    );
    function_pointers.push({
        let _ = 409usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "tdotz", 1047977935i64)
    },
    );
    function_pointers.push({
        let _ = 410usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "slerp", 3118673011i64)
    },
    );
    function_pointers.push({
        let _ = 411usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "is_conformal", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 412usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "is_equal_approx", 3165333982i64)
    },
    );
    function_pointers.push({
        let _ = 413usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "is_finite", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 414usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "get_rotation_quaternion", 4274879941i64)
    },
    );
    function_pointers.push({
        let _ = 415usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "looking_at", 3728732505i64)
    },
    );
    function_pointers.push({
        let _ = 416usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "from_scale", 3703240166i64)
    },
    );
    function_pointers.push({
        let _ = 417usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_BASIS, "Basis", "from_euler", 2802321791i64)
    },
    );
    
}
fn load_Transform3D_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 418usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, "Transform3D", "inverse", 3816817146i64)
    },
    );
    function_pointers.push({
        let _ = 419usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, "Transform3D", "affine_inverse", 3816817146i64)
    },
    );
    function_pointers.push({
        let _ = 420usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, "Transform3D", "orthonormalized", 3816817146i64)
    },
    );
    function_pointers.push({
        let _ = 421usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, "Transform3D", "rotated", 1563203923i64)
    },
    );
    function_pointers.push({
        let _ = 422usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, "Transform3D", "rotated_local", 1563203923i64)
    },
    );
    function_pointers.push({
        let _ = 423usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, "Transform3D", "scaled", 1405596198i64)
    },
    );
    function_pointers.push({
        let _ = 424usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, "Transform3D", "scaled_local", 1405596198i64)
    },
    );
    function_pointers.push({
        let _ = 425usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, "Transform3D", "translated", 1405596198i64)
    },
    );
    function_pointers.push({
        let _ = 426usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, "Transform3D", "translated_local", 1405596198i64)
    },
    );
    function_pointers.push({
        let _ = 427usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, "Transform3D", "looking_at", 90889270i64)
    },
    );
    function_pointers.push({
        let _ = 428usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, "Transform3D", "interpolate_with", 1786453358i64)
    },
    );
    function_pointers.push({
        let _ = 429usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, "Transform3D", "is_equal_approx", 696001652i64)
    },
    );
    function_pointers.push({
        let _ = 430usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, "Transform3D", "is_finite", 3918633141i64)
    },
    );
    
}
fn load_Projection_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 431usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "create_depth_correction", 1228516048i64)
    },
    );
    function_pointers.push({
        let _ = 432usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "create_light_atlas_rect", 2654950662i64)
    },
    );
    function_pointers.push({
        let _ = 433usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "create_perspective", 390915442i64)
    },
    );
    function_pointers.push({
        let _ = 434usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "create_perspective_hmd", 2857674800i64)
    },
    );
    function_pointers.push({
        let _ = 435usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "create_for_hmd", 4184144994i64)
    },
    );
    function_pointers.push({
        let _ = 436usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "create_orthogonal", 3707929169i64)
    },
    );
    function_pointers.push({
        let _ = 437usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "create_orthogonal_aspect", 390915442i64)
    },
    );
    function_pointers.push({
        let _ = 438usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "create_frustum", 3707929169i64)
    },
    );
    function_pointers.push({
        let _ = 439usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "create_frustum_aspect", 1535076251i64)
    },
    );
    function_pointers.push({
        let _ = 440usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "create_fit_aabb", 2264694907i64)
    },
    );
    function_pointers.push({
        let _ = 441usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "determinant", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 442usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "perspective_znear_adjusted", 3584785443i64)
    },
    );
    function_pointers.push({
        let _ = 443usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "get_projection_plane", 1551184160i64)
    },
    );
    function_pointers.push({
        let _ = 444usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "flipped_y", 4212530932i64)
    },
    );
    function_pointers.push({
        let _ = 445usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "jitter_offseted", 2448438599i64)
    },
    );
    function_pointers.push({
        let _ = 446usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "get_fovy", 3514207532i64)
    },
    );
    function_pointers.push({
        let _ = 447usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "get_z_far", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 448usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "get_z_near", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 449usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "get_aspect", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 450usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "get_fov", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 451usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "is_orthogonal", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 452usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "get_viewport_half_extents", 2428350749i64)
    },
    );
    function_pointers.push({
        let _ = 453usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "get_far_plane_half_extents", 2428350749i64)
    },
    );
    function_pointers.push({
        let _ = 454usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "inverse", 4212530932i64)
    },
    );
    function_pointers.push({
        let _ = 455usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "get_pixels_per_meter", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 456usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, "Projection", "get_lod_multiplier", 466405837i64)
    },
    );
    
}
fn load_Color_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 457usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "to_argb32", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 458usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "to_abgr32", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 459usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "to_rgba32", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 460usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "to_argb64", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 461usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "to_abgr64", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 462usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "to_rgba64", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 463usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "to_html", 3429816538i64)
    },
    );
    function_pointers.push({
        let _ = 464usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "clamp", 105651410i64)
    },
    );
    function_pointers.push({
        let _ = 465usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "inverted", 3334027602i64)
    },
    );
    function_pointers.push({
        let _ = 466usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "lerp", 402949615i64)
    },
    );
    function_pointers.push({
        let _ = 467usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "lightened", 1466039168i64)
    },
    );
    function_pointers.push({
        let _ = 468usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "darkened", 1466039168i64)
    },
    );
    function_pointers.push({
        let _ = 469usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "blend", 3803690977i64)
    },
    );
    function_pointers.push({
        let _ = 470usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "get_luminance", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 471usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "srgb_to_linear", 3334027602i64)
    },
    );
    function_pointers.push({
        let _ = 472usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "linear_to_srgb", 3334027602i64)
    },
    );
    function_pointers.push({
        let _ = 473usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "is_equal_approx", 3167426256i64)
    },
    );
    function_pointers.push({
        let _ = 474usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "hex", 351421375i64)
    },
    );
    function_pointers.push({
        let _ = 475usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "hex64", 351421375i64)
    },
    );
    function_pointers.push({
        let _ = 476usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "html", 2500054655i64)
    },
    );
    function_pointers.push({
        let _ = 477usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "html_is_valid", 2942997125i64)
    },
    );
    function_pointers.push({
        let _ = 478usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "from_string", 3755044230i64)
    },
    );
    function_pointers.push({
        let _ = 479usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "from_hsv", 1573799446i64)
    },
    );
    function_pointers.push({
        let _ = 480usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "from_ok_hsl", 1573799446i64)
    },
    );
    function_pointers.push({
        let _ = 481usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "from_rgbe9995", 351421375i64)
    },
    );
    function_pointers.push({
        let _ = 482usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_COLOR, "Color", "from_rgba8", 3072934735i64)
    },
    );
    
}
fn load_StringName_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 483usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "casecmp_to", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 484usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "nocasecmp_to", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 485usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "naturalcasecmp_to", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 486usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "naturalnocasecmp_to", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 487usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "filecasecmp_to", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 488usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "filenocasecmp_to", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 489usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "length", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 490usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "substr", 787537301i64)
    },
    );
    function_pointers.push({
        let _ = 491usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "get_slice", 3535100402i64)
    },
    );
    function_pointers.push({
        let _ = 492usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "get_slicec", 787537301i64)
    },
    );
    function_pointers.push({
        let _ = 493usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "get_slice_count", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 494usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "find", 1760645412i64)
    },
    );
    function_pointers.push({
        let _ = 495usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "findn", 1760645412i64)
    },
    );
    function_pointers.push({
        let _ = 496usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "count", 2343087891i64)
    },
    );
    function_pointers.push({
        let _ = 497usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "countn", 2343087891i64)
    },
    );
    function_pointers.push({
        let _ = 498usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "rfind", 1760645412i64)
    },
    );
    function_pointers.push({
        let _ = 499usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "rfindn", 1760645412i64)
    },
    );
    function_pointers.push({
        let _ = 500usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "match", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 501usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "matchn", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 502usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "begins_with", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 503usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "ends_with", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 504usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_subsequence_of", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 505usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_subsequence_ofn", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 506usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "bigrams", 747180633i64)
    },
    );
    function_pointers.push({
        let _ = 507usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "similarity", 2697460964i64)
    },
    );
    function_pointers.push({
        let _ = 508usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "format", 3212199029i64)
    },
    );
    function_pointers.push({
        let _ = 509usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "replace", 1340436205i64)
    },
    );
    function_pointers.push({
        let _ = 510usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "replacen", 1340436205i64)
    },
    );
    function_pointers.push({
        let _ = 511usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "replace_char", 787537301i64)
    },
    );
    function_pointers.push({
        let _ = 512usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "replace_chars", 3535100402i64)
    },
    );
    function_pointers.push({
        let _ = 513usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "remove_char", 2162347432i64)
    },
    );
    function_pointers.push({
        let _ = 514usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "remove_chars", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 515usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "repeat", 2162347432i64)
    },
    );
    function_pointers.push({
        let _ = 516usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "reverse", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 517usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "insert", 248737229i64)
    },
    );
    function_pointers.push({
        let _ = 518usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "erase", 787537301i64)
    },
    );
    function_pointers.push({
        let _ = 519usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "capitalize", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 520usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_camel_case", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 521usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_pascal_case", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 522usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_snake_case", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 523usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_kebab_case", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 524usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "split", 1252735785i64)
    },
    );
    function_pointers.push({
        let _ = 525usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "rsplit", 1252735785i64)
    },
    );
    function_pointers.push({
        let _ = 526usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "split_floats", 2092079095i64)
    },
    );
    function_pointers.push({
        let _ = 527usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "join", 3595973238i64)
    },
    );
    function_pointers.push({
        let _ = 528usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_upper", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 529usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_lower", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 530usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "left", 2162347432i64)
    },
    );
    function_pointers.push({
        let _ = 531usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "right", 2162347432i64)
    },
    );
    function_pointers.push({
        let _ = 532usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "strip_edges", 907855311i64)
    },
    );
    function_pointers.push({
        let _ = 533usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "strip_escapes", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 534usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "lstrip", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 535usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "rstrip", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 536usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "get_extension", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 537usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "get_basename", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 538usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "path_join", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 539usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "unicode_at", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 540usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "indent", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 541usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "dedent", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 542usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "md5_text", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 543usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "sha1_text", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 544usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "sha256_text", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 545usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "md5_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 546usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "sha1_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 547usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "sha256_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 548usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 549usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "contains", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 550usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "containsn", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 551usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_absolute_path", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 552usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_relative_path", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 553usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "simplify_path", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 554usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "get_base_dir", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 555usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "get_file", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 556usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "xml_escape", 3429816538i64)
    },
    );
    function_pointers.push({
        let _ = 557usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "xml_unescape", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 558usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "uri_encode", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 559usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "uri_decode", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 560usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "uri_file_decode", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 561usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "c_escape", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 562usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "c_unescape", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 563usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "json_escape", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 564usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "validate_node_name", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 565usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "validate_filename", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 566usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_valid_ascii_identifier", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 567usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_valid_unicode_identifier", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 568usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_valid_identifier", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 569usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_valid_int", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 570usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_valid_float", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 571usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_valid_hex_number", 593672999i64)
    },
    );
    function_pointers.push({
        let _ = 572usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_valid_html_color", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 573usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_valid_ip_address", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 574usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "is_valid_filename", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 575usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_int", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 576usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_float", 466405837i64)
    },
    );
    function_pointers.push({
        let _ = 577usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "hex_to_int", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 578usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "bin_to_int", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 579usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "lpad", 248737229i64)
    },
    );
    function_pointers.push({
        let _ = 580usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "rpad", 248737229i64)
    },
    );
    function_pointers.push({
        let _ = 581usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "pad_decimals", 2162347432i64)
    },
    );
    function_pointers.push({
        let _ = 582usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "pad_zeros", 2162347432i64)
    },
    );
    function_pointers.push({
        let _ = 583usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "trim_prefix", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 584usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "trim_suffix", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 585usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_ascii_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 586usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_utf8_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 587usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_utf16_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 588usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_utf32_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 589usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_wchar_buffer", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 590usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "to_multibyte_char_buffer", 3055765187i64)
    },
    );
    function_pointers.push({
        let _ = 591usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "hex_decode", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 592usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, "StringName", "hash", 3173160232i64)
    },
    );
    
}
fn load_NodePath_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 593usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, "NodePath", "is_absolute", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 594usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, "NodePath", "get_name_count", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 595usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, "NodePath", "get_name", 2948586938i64)
    },
    );
    function_pointers.push({
        let _ = 596usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, "NodePath", "get_subname_count", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 597usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, "NodePath", "hash", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 598usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, "NodePath", "get_subname", 2948586938i64)
    },
    );
    function_pointers.push({
        let _ = 599usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, "NodePath", "get_concatenated_names", 1825232092i64)
    },
    );
    function_pointers.push({
        let _ = 600usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, "NodePath", "get_concatenated_subnames", 1825232092i64)
    },
    );
    function_pointers.push({
        let _ = 601usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, "NodePath", "slice", 421628484i64)
    },
    );
    function_pointers.push({
        let _ = 602usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, "NodePath", "get_as_property_path", 1598598043i64)
    },
    );
    function_pointers.push({
        let _ = 603usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, "NodePath", "is_empty", 3918633141i64)
    },
    );
    
}
fn load_RID_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 604usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RID, "RID", "is_valid", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 605usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_RID, "RID", "get_id", 3173160232i64)
    },
    );
    
}
fn load_Callable_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 606usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "create", 1709381114i64)
    },
    );
    function_pointers.push({
        let _ = 607usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "callv", 413578926i64)
    },
    );
    function_pointers.push({
        let _ = 608usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "is_null", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 609usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "is_custom", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 610usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "is_standard", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 611usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "is_valid", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 612usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "get_object", 4008621732i64)
    },
    );
    function_pointers.push({
        let _ = 613usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "get_object_id", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 614usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "get_method", 1825232092i64)
    },
    );
    function_pointers.push({
        let _ = 615usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "get_argument_count", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 616usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "get_bound_arguments_count", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 617usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "get_bound_arguments", 4144163970i64)
    },
    );
    function_pointers.push({
        let _ = 618usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "get_unbound_arguments_count", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 619usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "hash", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 620usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "bindv", 3564560322i64)
    },
    );
    function_pointers.push({
        let _ = 621usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "unbind", 755001590i64)
    },
    );
    function_pointers.push({
        let _ = 622usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "call", 3643564216i64)
    },
    );
    function_pointers.push({
        let _ = 623usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "call_deferred", 3286317445i64)
    },
    );
    function_pointers.push({
        let _ = 624usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "rpc", 3286317445i64)
    },
    );
    function_pointers.push({
        let _ = 625usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "rpc_id", 2270047679i64)
    },
    );
    function_pointers.push({
        let _ = 626usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, "Callable", "bind", 3224143119i64)
    },
    );
    
}
fn load_Signal_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 627usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, "Signal", "is_null", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 628usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, "Signal", "get_object", 4008621732i64)
    },
    );
    function_pointers.push({
        let _ = 629usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, "Signal", "get_object_id", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 630usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, "Signal", "get_name", 1825232092i64)
    },
    );
    function_pointers.push({
        let _ = 631usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, "Signal", "connect", 979702392i64)
    },
    );
    function_pointers.push({
        let _ = 632usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, "Signal", "disconnect", 3470848906i64)
    },
    );
    function_pointers.push({
        let _ = 633usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, "Signal", "is_connected", 4129521963i64)
    },
    );
    function_pointers.push({
        let _ = 634usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, "Signal", "get_connections", 4144163970i64)
    },
    );
    function_pointers.push({
        let _ = 635usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, "Signal", "has_connections", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 636usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, "Signal", "emit", 3286317445i64)
    },
    );
    
}
fn load_Dictionary_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 637usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "size", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 638usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 639usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "clear", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 640usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "assign", 3642266950i64)
    },
    );
    function_pointers.push({
        let _ = 641usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "sort", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 642usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "merge", 2079548978i64)
    },
    );
    function_pointers.push({
        let _ = 643usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "merged", 2271165639i64)
    },
    );
    function_pointers.push({
        let _ = 644usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "has", 3680194679i64)
    },
    );
    function_pointers.push({
        let _ = 645usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "has_all", 2988181878i64)
    },
    );
    function_pointers.push({
        let _ = 646usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "find_key", 1988825835i64)
    },
    );
    function_pointers.push({
        let _ = 647usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "erase", 1776646889i64)
    },
    );
    function_pointers.push({
        let _ = 648usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "hash", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 649usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "keys", 4144163970i64)
    },
    );
    function_pointers.push({
        let _ = 650usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "values", 4144163970i64)
    },
    );
    function_pointers.push({
        let _ = 651usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "duplicate", 830099069i64)
    },
    );
    function_pointers.push({
        let _ = 652usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "duplicate_deep", 2160600714i64)
    },
    );
    function_pointers.push({
        let _ = 653usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "get", 2205440559i64)
    },
    );
    function_pointers.push({
        let _ = 654usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "get_or_add", 1052551076i64)
    },
    );
    function_pointers.push({
        let _ = 655usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "set", 2175348267i64)
    },
    );
    function_pointers.push({
        let _ = 656usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "is_typed", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 657usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "is_typed_key", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 658usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "is_typed_value", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 659usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "is_same_typed", 3471775634i64)
    },
    );
    function_pointers.push({
        let _ = 660usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "is_same_typed_key", 3471775634i64)
    },
    );
    function_pointers.push({
        let _ = 661usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "is_same_typed_value", 3471775634i64)
    },
    );
    function_pointers.push({
        let _ = 662usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "get_typed_key_builtin", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 663usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "get_typed_value_builtin", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 664usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "get_typed_key_class_name", 1825232092i64)
    },
    );
    function_pointers.push({
        let _ = 665usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "get_typed_value_class_name", 1825232092i64)
    },
    );
    function_pointers.push({
        let _ = 666usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "get_typed_key_script", 1460142086i64)
    },
    );
    function_pointers.push({
        let _ = 667usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "get_typed_value_script", 1460142086i64)
    },
    );
    function_pointers.push({
        let _ = 668usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "make_read_only", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 669usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "is_read_only", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 670usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, "Dictionary", "recursive_equal", 1404404751i64)
    },
    );
    
}
fn load_Array_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 671usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "size", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 672usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 673usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "clear", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 674usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "hash", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 675usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "assign", 2307260970i64)
    },
    );
    function_pointers.push({
        let _ = 676usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "get", 708700221i64)
    },
    );
    function_pointers.push({
        let _ = 677usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "set", 3798478031i64)
    },
    );
    function_pointers.push({
        let _ = 678usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "push_back", 3316032543i64)
    },
    );
    function_pointers.push({
        let _ = 679usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "push_front", 3316032543i64)
    },
    );
    function_pointers.push({
        let _ = 680usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "append", 3316032543i64)
    },
    );
    function_pointers.push({
        let _ = 681usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "append_array", 2307260970i64)
    },
    );
    function_pointers.push({
        let _ = 682usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "resize", 848867239i64)
    },
    );
    function_pointers.push({
        let _ = 683usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "insert", 3176316662i64)
    },
    );
    function_pointers.push({
        let _ = 684usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "remove_at", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 685usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "fill", 3316032543i64)
    },
    );
    function_pointers.push({
        let _ = 686usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "erase", 3316032543i64)
    },
    );
    function_pointers.push({
        let _ = 687usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "front", 1460142086i64)
    },
    );
    function_pointers.push({
        let _ = 688usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "back", 1460142086i64)
    },
    );
    function_pointers.push({
        let _ = 689usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "pick_random", 1460142086i64)
    },
    );
    function_pointers.push({
        let _ = 690usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "find", 2336346817i64)
    },
    );
    function_pointers.push({
        let _ = 691usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "find_custom", 2145562546i64)
    },
    );
    function_pointers.push({
        let _ = 692usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "rfind", 2336346817i64)
    },
    );
    function_pointers.push({
        let _ = 693usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "rfind_custom", 2145562546i64)
    },
    );
    function_pointers.push({
        let _ = 694usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "count", 1481661226i64)
    },
    );
    function_pointers.push({
        let _ = 695usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "has", 3680194679i64)
    },
    );
    function_pointers.push({
        let _ = 696usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "pop_back", 1321915136i64)
    },
    );
    function_pointers.push({
        let _ = 697usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "pop_front", 1321915136i64)
    },
    );
    function_pointers.push({
        let _ = 698usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "pop_at", 3518259424i64)
    },
    );
    function_pointers.push({
        let _ = 699usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "sort", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 700usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "sort_custom", 3470848906i64)
    },
    );
    function_pointers.push({
        let _ = 701usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "shuffle", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 702usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "bsearch", 3372222236i64)
    },
    );
    function_pointers.push({
        let _ = 703usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "bsearch_custom", 161317131i64)
    },
    );
    function_pointers.push({
        let _ = 704usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "reverse", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 705usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "duplicate", 636440122i64)
    },
    );
    function_pointers.push({
        let _ = 706usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "duplicate_deep", 1949240801i64)
    },
    );
    function_pointers.push({
        let _ = 707usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "slice", 1393718243i64)
    },
    );
    function_pointers.push({
        let _ = 708usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "filter", 4075186556i64)
    },
    );
    function_pointers.push({
        let _ = 709usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "map", 4075186556i64)
    },
    );
    function_pointers.push({
        let _ = 710usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "reduce", 4272450342i64)
    },
    );
    function_pointers.push({
        let _ = 711usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "any", 4129521963i64)
    },
    );
    function_pointers.push({
        let _ = 712usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "all", 4129521963i64)
    },
    );
    function_pointers.push({
        let _ = 713usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "max", 1460142086i64)
    },
    );
    function_pointers.push({
        let _ = 714usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "min", 1460142086i64)
    },
    );
    function_pointers.push({
        let _ = 715usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "is_typed", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 716usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "is_same_typed", 2988181878i64)
    },
    );
    function_pointers.push({
        let _ = 717usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "get_typed_builtin", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 718usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "get_typed_class_name", 1825232092i64)
    },
    );
    function_pointers.push({
        let _ = 719usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "get_typed_script", 1460142086i64)
    },
    );
    function_pointers.push({
        let _ = 720usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "make_read_only", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 721usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, "Array", "is_read_only", 3918633141i64)
    },
    );
    
}
fn load_PackedByteArray_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 722usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "get", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 723usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "set", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 724usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "size", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 725usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 726usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "push_back", 694024632i64)
    },
    );
    function_pointers.push({
        let _ = 727usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "append", 694024632i64)
    },
    );
    function_pointers.push({
        let _ = 728usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "append_array", 791097111i64)
    },
    );
    function_pointers.push({
        let _ = 729usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "remove_at", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 730usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "insert", 1487112728i64)
    },
    );
    function_pointers.push({
        let _ = 731usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "fill", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 732usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "resize", 848867239i64)
    },
    );
    function_pointers.push({
        let _ = 733usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "clear", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 734usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "has", 931488181i64)
    },
    );
    function_pointers.push({
        let _ = 735usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "reverse", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 736usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "slice", 2278869132i64)
    },
    );
    function_pointers.push({
        let _ = 737usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "sort", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 738usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "bsearch", 3380005890i64)
    },
    );
    function_pointers.push({
        let _ = 739usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "duplicate", 851781288i64)
    },
    );
    function_pointers.push({
        let _ = 740usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "find", 2984303840i64)
    },
    );
    function_pointers.push({
        let _ = 741usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "rfind", 2984303840i64)
    },
    );
    function_pointers.push({
        let _ = 742usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "count", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 743usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "erase", 694024632i64)
    },
    );
    function_pointers.push({
        let _ = 744usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "get_string_from_ascii", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 745usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "get_string_from_utf8", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 746usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "get_string_from_utf16", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 747usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "get_string_from_utf32", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 748usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "get_string_from_wchar", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 749usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "get_string_from_multibyte_char", 3134094431i64)
    },
    );
    function_pointers.push({
        let _ = 750usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "hex_encode", 3942272618i64)
    },
    );
    function_pointers.push({
        let _ = 751usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "compress", 1845905913i64)
    },
    );
    function_pointers.push({
        let _ = 752usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decompress", 2278869132i64)
    },
    );
    function_pointers.push({
        let _ = 753usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decompress_dynamic", 2278869132i64)
    },
    );
    function_pointers.push({
        let _ = 754usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decode_u8", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 755usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decode_s8", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 756usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decode_u16", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 757usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decode_s16", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 758usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decode_u32", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 759usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decode_s32", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 760usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decode_u64", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 761usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decode_s64", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 762usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decode_half", 1401583798i64)
    },
    );
    function_pointers.push({
        let _ = 763usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decode_float", 1401583798i64)
    },
    );
    function_pointers.push({
        let _ = 764usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decode_double", 1401583798i64)
    },
    );
    function_pointers.push({
        let _ = 765usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "has_encoded_var", 2914632957i64)
    },
    );
    function_pointers.push({
        let _ = 766usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decode_var", 1740420038i64)
    },
    );
    function_pointers.push({
        let _ = 767usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "decode_var_size", 954237325i64)
    },
    );
    function_pointers.push({
        let _ = 768usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "to_int32_array", 3158844420i64)
    },
    );
    function_pointers.push({
        let _ = 769usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "to_int64_array", 1961294120i64)
    },
    );
    function_pointers.push({
        let _ = 770usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "to_float32_array", 3575107827i64)
    },
    );
    function_pointers.push({
        let _ = 771usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "to_float64_array", 1627308337i64)
    },
    );
    function_pointers.push({
        let _ = 772usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "to_vector2_array", 1660374357i64)
    },
    );
    function_pointers.push({
        let _ = 773usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "to_vector3_array", 4171207452i64)
    },
    );
    function_pointers.push({
        let _ = 774usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "to_vector4_array", 146203628i64)
    },
    );
    function_pointers.push({
        let _ = 775usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "to_color_array", 3072026941i64)
    },
    );
    function_pointers.push({
        let _ = 776usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "bswap16", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 777usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "bswap32", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 778usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "bswap64", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 779usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "encode_u8", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 780usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "encode_s8", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 781usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "encode_u16", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 782usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "encode_s16", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 783usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "encode_u32", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 784usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "encode_s32", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 785usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "encode_u64", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 786usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "encode_s64", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 787usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "encode_half", 1113000516i64)
    },
    );
    function_pointers.push({
        let _ = 788usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "encode_float", 1113000516i64)
    },
    );
    function_pointers.push({
        let _ = 789usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "encode_double", 1113000516i64)
    },
    );
    function_pointers.push({
        let _ = 790usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, "PackedByteArray", "encode_var", 2604460497i64)
    },
    );
    
}
fn load_PackedInt32Array_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 791usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "get", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 792usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "set", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 793usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "size", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 794usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 795usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "push_back", 694024632i64)
    },
    );
    function_pointers.push({
        let _ = 796usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "append", 694024632i64)
    },
    );
    function_pointers.push({
        let _ = 797usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "append_array", 1087733270i64)
    },
    );
    function_pointers.push({
        let _ = 798usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "remove_at", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 799usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "insert", 1487112728i64)
    },
    );
    function_pointers.push({
        let _ = 800usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "fill", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 801usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "resize", 848867239i64)
    },
    );
    function_pointers.push({
        let _ = 802usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "clear", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 803usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "has", 931488181i64)
    },
    );
    function_pointers.push({
        let _ = 804usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "reverse", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 805usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "slice", 1216021098i64)
    },
    );
    function_pointers.push({
        let _ = 806usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "to_byte_array", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 807usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "sort", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 808usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "bsearch", 3380005890i64)
    },
    );
    function_pointers.push({
        let _ = 809usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "duplicate", 1997843129i64)
    },
    );
    function_pointers.push({
        let _ = 810usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "find", 2984303840i64)
    },
    );
    function_pointers.push({
        let _ = 811usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "rfind", 2984303840i64)
    },
    );
    function_pointers.push({
        let _ = 812usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "count", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 813usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, "PackedInt32Array", "erase", 694024632i64)
    },
    );
    
}
fn load_PackedInt64Array_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 814usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "get", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 815usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "set", 3638975848i64)
    },
    );
    function_pointers.push({
        let _ = 816usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "size", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 817usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 818usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "push_back", 694024632i64)
    },
    );
    function_pointers.push({
        let _ = 819usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "append", 694024632i64)
    },
    );
    function_pointers.push({
        let _ = 820usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "append_array", 2090311302i64)
    },
    );
    function_pointers.push({
        let _ = 821usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "remove_at", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 822usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "insert", 1487112728i64)
    },
    );
    function_pointers.push({
        let _ = 823usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "fill", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 824usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "resize", 848867239i64)
    },
    );
    function_pointers.push({
        let _ = 825usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "clear", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 826usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "has", 931488181i64)
    },
    );
    function_pointers.push({
        let _ = 827usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "reverse", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 828usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "slice", 1726550804i64)
    },
    );
    function_pointers.push({
        let _ = 829usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "to_byte_array", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 830usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "sort", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 831usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "bsearch", 3380005890i64)
    },
    );
    function_pointers.push({
        let _ = 832usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "duplicate", 2376370016i64)
    },
    );
    function_pointers.push({
        let _ = 833usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "find", 2984303840i64)
    },
    );
    function_pointers.push({
        let _ = 834usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "rfind", 2984303840i64)
    },
    );
    function_pointers.push({
        let _ = 835usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "count", 4103005248i64)
    },
    );
    function_pointers.push({
        let _ = 836usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, "PackedInt64Array", "erase", 694024632i64)
    },
    );
    
}
fn load_PackedFloat32Array_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 837usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "get", 1401583798i64)
    },
    );
    function_pointers.push({
        let _ = 838usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "set", 1113000516i64)
    },
    );
    function_pointers.push({
        let _ = 839usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "size", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 840usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 841usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "push_back", 4094791666i64)
    },
    );
    function_pointers.push({
        let _ = 842usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "append", 4094791666i64)
    },
    );
    function_pointers.push({
        let _ = 843usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "append_array", 2981316639i64)
    },
    );
    function_pointers.push({
        let _ = 844usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "remove_at", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 845usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "insert", 1379903876i64)
    },
    );
    function_pointers.push({
        let _ = 846usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "fill", 833936903i64)
    },
    );
    function_pointers.push({
        let _ = 847usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "resize", 848867239i64)
    },
    );
    function_pointers.push({
        let _ = 848usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "clear", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 849usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "has", 1296369134i64)
    },
    );
    function_pointers.push({
        let _ = 850usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "reverse", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 851usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "slice", 1418229160i64)
    },
    );
    function_pointers.push({
        let _ = 852usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "to_byte_array", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 853usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "sort", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 854usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "bsearch", 1188816338i64)
    },
    );
    function_pointers.push({
        let _ = 855usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "duplicate", 831114784i64)
    },
    );
    function_pointers.push({
        let _ = 856usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "find", 1343150241i64)
    },
    );
    function_pointers.push({
        let _ = 857usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "rfind", 1343150241i64)
    },
    );
    function_pointers.push({
        let _ = 858usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "count", 2859915090i64)
    },
    );
    function_pointers.push({
        let _ = 859usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, "PackedFloat32Array", "erase", 4094791666i64)
    },
    );
    
}
fn load_PackedFloat64Array_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 860usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "get", 1401583798i64)
    },
    );
    function_pointers.push({
        let _ = 861usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "set", 1113000516i64)
    },
    );
    function_pointers.push({
        let _ = 862usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "size", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 863usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 864usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "push_back", 4094791666i64)
    },
    );
    function_pointers.push({
        let _ = 865usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "append", 4094791666i64)
    },
    );
    function_pointers.push({
        let _ = 866usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "append_array", 792078629i64)
    },
    );
    function_pointers.push({
        let _ = 867usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "remove_at", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 868usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "insert", 1379903876i64)
    },
    );
    function_pointers.push({
        let _ = 869usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "fill", 833936903i64)
    },
    );
    function_pointers.push({
        let _ = 870usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "resize", 848867239i64)
    },
    );
    function_pointers.push({
        let _ = 871usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "clear", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 872usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "has", 1296369134i64)
    },
    );
    function_pointers.push({
        let _ = 873usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "reverse", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 874usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "slice", 2192974324i64)
    },
    );
    function_pointers.push({
        let _ = 875usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "to_byte_array", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 876usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "sort", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 877usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "bsearch", 1188816338i64)
    },
    );
    function_pointers.push({
        let _ = 878usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "duplicate", 949266573i64)
    },
    );
    function_pointers.push({
        let _ = 879usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "find", 1343150241i64)
    },
    );
    function_pointers.push({
        let _ = 880usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "rfind", 1343150241i64)
    },
    );
    function_pointers.push({
        let _ = 881usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "count", 2859915090i64)
    },
    );
    function_pointers.push({
        let _ = 882usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, "PackedFloat64Array", "erase", 4094791666i64)
    },
    );
    
}
fn load_PackedStringArray_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 883usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "get", 2162347432i64)
    },
    );
    function_pointers.push({
        let _ = 884usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "set", 725585539i64)
    },
    );
    function_pointers.push({
        let _ = 885usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "size", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 886usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 887usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "push_back", 816187996i64)
    },
    );
    function_pointers.push({
        let _ = 888usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "append", 816187996i64)
    },
    );
    function_pointers.push({
        let _ = 889usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "append_array", 1120103966i64)
    },
    );
    function_pointers.push({
        let _ = 890usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "remove_at", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 891usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "insert", 2432393153i64)
    },
    );
    function_pointers.push({
        let _ = 892usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "fill", 3174917410i64)
    },
    );
    function_pointers.push({
        let _ = 893usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "resize", 848867239i64)
    },
    );
    function_pointers.push({
        let _ = 894usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "clear", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 895usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "has", 2566493496i64)
    },
    );
    function_pointers.push({
        let _ = 896usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "reverse", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 897usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "slice", 2094601407i64)
    },
    );
    function_pointers.push({
        let _ = 898usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "to_byte_array", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 899usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "sort", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 900usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "bsearch", 328976671i64)
    },
    );
    function_pointers.push({
        let _ = 901usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "duplicate", 2991231410i64)
    },
    );
    function_pointers.push({
        let _ = 902usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "find", 1760645412i64)
    },
    );
    function_pointers.push({
        let _ = 903usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "rfind", 1760645412i64)
    },
    );
    function_pointers.push({
        let _ = 904usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "count", 2920860731i64)
    },
    );
    function_pointers.push({
        let _ = 905usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, "PackedStringArray", "erase", 816187996i64)
    },
    );
    
}
fn load_PackedVector2Array_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 906usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "get", 2609058838i64)
    },
    );
    function_pointers.push({
        let _ = 907usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "set", 635767250i64)
    },
    );
    function_pointers.push({
        let _ = 908usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "size", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 909usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 910usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "push_back", 4188891560i64)
    },
    );
    function_pointers.push({
        let _ = 911usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "append", 4188891560i64)
    },
    );
    function_pointers.push({
        let _ = 912usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "append_array", 3887534835i64)
    },
    );
    function_pointers.push({
        let _ = 913usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "remove_at", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 914usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "insert", 2225629369i64)
    },
    );
    function_pointers.push({
        let _ = 915usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "fill", 3790411178i64)
    },
    );
    function_pointers.push({
        let _ = 916usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "resize", 848867239i64)
    },
    );
    function_pointers.push({
        let _ = 917usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "clear", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 918usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "has", 3190634762i64)
    },
    );
    function_pointers.push({
        let _ = 919usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "reverse", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 920usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "slice", 3864005350i64)
    },
    );
    function_pointers.push({
        let _ = 921usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "to_byte_array", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 922usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "sort", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 923usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "bsearch", 3778035805i64)
    },
    );
    function_pointers.push({
        let _ = 924usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "duplicate", 3763646812i64)
    },
    );
    function_pointers.push({
        let _ = 925usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "find", 1469606149i64)
    },
    );
    function_pointers.push({
        let _ = 926usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "rfind", 1469606149i64)
    },
    );
    function_pointers.push({
        let _ = 927usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "count", 2798848307i64)
    },
    );
    function_pointers.push({
        let _ = 928usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, "PackedVector2Array", "erase", 4188891560i64)
    },
    );
    
}
fn load_PackedVector3Array_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 929usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "get", 1394941017i64)
    },
    );
    function_pointers.push({
        let _ = 930usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "set", 3975343409i64)
    },
    );
    function_pointers.push({
        let _ = 931usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "size", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 932usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 933usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "push_back", 3295363524i64)
    },
    );
    function_pointers.push({
        let _ = 934usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "append", 3295363524i64)
    },
    );
    function_pointers.push({
        let _ = 935usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "append_array", 203538016i64)
    },
    );
    function_pointers.push({
        let _ = 936usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "remove_at", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 937usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "insert", 3892262309i64)
    },
    );
    function_pointers.push({
        let _ = 938usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "fill", 3726392409i64)
    },
    );
    function_pointers.push({
        let _ = 939usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "resize", 848867239i64)
    },
    );
    function_pointers.push({
        let _ = 940usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "clear", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 941usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "has", 1749054343i64)
    },
    );
    function_pointers.push({
        let _ = 942usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "reverse", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 943usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "slice", 2086131305i64)
    },
    );
    function_pointers.push({
        let _ = 944usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "to_byte_array", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 945usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "sort", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 946usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "bsearch", 219263630i64)
    },
    );
    function_pointers.push({
        let _ = 947usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "duplicate", 2754175465i64)
    },
    );
    function_pointers.push({
        let _ = 948usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "find", 3718155780i64)
    },
    );
    function_pointers.push({
        let _ = 949usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "rfind", 3718155780i64)
    },
    );
    function_pointers.push({
        let _ = 950usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "count", 194580386i64)
    },
    );
    function_pointers.push({
        let _ = 951usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, "PackedVector3Array", "erase", 3295363524i64)
    },
    );
    
}
fn load_PackedColorArray_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 952usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "get", 2972831132i64)
    },
    );
    function_pointers.push({
        let _ = 953usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "set", 1444096570i64)
    },
    );
    function_pointers.push({
        let _ = 954usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "size", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 955usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 956usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "push_back", 1007858200i64)
    },
    );
    function_pointers.push({
        let _ = 957usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "append", 1007858200i64)
    },
    );
    function_pointers.push({
        let _ = 958usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "append_array", 798822497i64)
    },
    );
    function_pointers.push({
        let _ = 959usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "remove_at", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 960usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "insert", 785289703i64)
    },
    );
    function_pointers.push({
        let _ = 961usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "fill", 3730314301i64)
    },
    );
    function_pointers.push({
        let _ = 962usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "resize", 848867239i64)
    },
    );
    function_pointers.push({
        let _ = 963usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "clear", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 964usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "has", 3167426256i64)
    },
    );
    function_pointers.push({
        let _ = 965usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "reverse", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 966usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "slice", 2451797139i64)
    },
    );
    function_pointers.push({
        let _ = 967usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "to_byte_array", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 968usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "sort", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 969usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "bsearch", 314143821i64)
    },
    );
    function_pointers.push({
        let _ = 970usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "duplicate", 1011903421i64)
    },
    );
    function_pointers.push({
        let _ = 971usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "find", 3156095363i64)
    },
    );
    function_pointers.push({
        let _ = 972usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "rfind", 3156095363i64)
    },
    );
    function_pointers.push({
        let _ = 973usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "count", 1682108616i64)
    },
    );
    function_pointers.push({
        let _ = 974usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, "PackedColorArray", "erase", 1007858200i64)
    },
    );
    
}
fn load_PackedVector4Array_methods(function_pointers: &mut Vec < crate::BuiltinMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    function_pointers.push({
        let _ = 975usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "get", 1227817084i64)
    },
    );
    function_pointers.push({
        let _ = 976usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "set", 1350366223i64)
    },
    );
    function_pointers.push({
        let _ = 977usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "size", 3173160232i64)
    },
    );
    function_pointers.push({
        let _ = 978usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "is_empty", 3918633141i64)
    },
    );
    function_pointers.push({
        let _ = 979usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "push_back", 3289167688i64)
    },
    );
    function_pointers.push({
        let _ = 980usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "append", 3289167688i64)
    },
    );
    function_pointers.push({
        let _ = 981usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "append_array", 537428395i64)
    },
    );
    function_pointers.push({
        let _ = 982usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "remove_at", 2823966027i64)
    },
    );
    function_pointers.push({
        let _ = 983usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "insert", 11085009i64)
    },
    );
    function_pointers.push({
        let _ = 984usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "fill", 3761353134i64)
    },
    );
    function_pointers.push({
        let _ = 985usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "resize", 848867239i64)
    },
    );
    function_pointers.push({
        let _ = 986usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "clear", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 987usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "has", 88913544i64)
    },
    );
    function_pointers.push({
        let _ = 988usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "reverse", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 989usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "slice", 2942803855i64)
    },
    );
    function_pointers.push({
        let _ = 990usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "to_byte_array", 247621236i64)
    },
    );
    function_pointers.push({
        let _ = 991usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "sort", 3218959716i64)
    },
    );
    function_pointers.push({
        let _ = 992usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "bsearch", 735671678i64)
    },
    );
    function_pointers.push({
        let _ = 993usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "duplicate", 3186305013i64)
    },
    );
    function_pointers.push({
        let _ = 994usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "find", 3091171314i64)
    },
    );
    function_pointers.push({
        let _ = 995usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "rfind", 3091171314i64)
    },
    );
    function_pointers.push({
        let _ = 996usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "count", 3956594488i64)
    },
    );
    function_pointers.push({
        let _ = 997usize;
        crate::load_builtin_method(fetch_fptr, string_names, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, "PackedVector4Array", "erase", 3289167688i64)
    },
    );
    
}