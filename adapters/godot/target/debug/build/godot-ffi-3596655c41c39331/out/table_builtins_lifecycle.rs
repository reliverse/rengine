use crate::{
    GDExtensionConstTypePtr, GDExtensionTypePtr, GDExtensionUninitializedTypePtr, GDExtensionUninitializedVariantPtr, GDExtensionVariantPtr,
};
#[allow(non_snake_case)]
pub struct BuiltinLifecycleTable {
    pub bool_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub bool_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub int_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub int_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub float_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub float_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub string_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub string_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub string_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub string_operator_less: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub string_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub string_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub string_from_string_name: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub string_from_node_path: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub string_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub vector2_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub vector2_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub vector2_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub vector2_operator_less: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub vector2_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector2_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector2_from_vector2i: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector2_from_x_y: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector2i_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub vector2i_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub vector2i_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub vector2i_operator_less: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub vector2i_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector2i_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector2i_from_vector2: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector2i_from_x_y: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub rect2_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub rect2_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub rect2_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub rect2_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub rect2_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub rect2_from_rect2i: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub rect2_from_position_size: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub rect2_from_x_y_width_height: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub rect2i_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub rect2i_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub rect2i_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub rect2i_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub rect2i_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub rect2i_from_rect2: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub rect2i_from_position_size: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub rect2i_from_x_y_width_height: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector3_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub vector3_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub vector3_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub vector3_operator_less: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub vector3_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector3_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector3_from_vector3i: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector3_from_x_y_z: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector3i_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub vector3i_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub vector3i_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub vector3i_operator_less: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub vector3i_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector3i_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector3i_from_vector3: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector3i_from_x_y_z: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub transform_2d_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub transform_2d_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub transform_2d_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub transform_2d_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub transform_2d_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub transform_2d_from_rotation_position: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub transform_2d_from_rotation_scale_skew_position: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub transform_2d_from_x_axis_y_axis_origin: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector4_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub vector4_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub vector4_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub vector4_operator_less: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub vector4_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector4_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector4_from_vector4i: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector4_from_x_y_z_w: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector4i_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub vector4i_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub vector4i_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub vector4i_operator_less: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub vector4i_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector4i_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector4i_from_vector4: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub vector4i_from_x_y_z_w: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub plane_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub plane_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub plane_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub plane_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub plane_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub plane_from_normal: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub plane_from_normal_d: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub plane_from_normal_point: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub plane_from_point1_point2_point3: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub plane_from_a_b_c_d: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub quaternion_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub quaternion_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub quaternion_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub quaternion_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub quaternion_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub quaternion_from_basis: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub quaternion_from_axis_angle: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub quaternion_from_arc_from_arc_to: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub quaternion_from_x_y_z_w: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub aabb_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub aabb_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub aabb_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub aabb_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub aabb_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub aabb_from_position_size: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub basis_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub basis_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub basis_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub basis_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub basis_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub basis_from_quaternion: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub basis_from_axis_angle: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub basis_from_x_axis_y_axis_z_axis: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub transform_3d_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub transform_3d_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub transform_3d_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub transform_3d_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub transform_3d_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub transform_3d_from_basis_origin: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub transform_3d_from_x_axis_y_axis_z_axis_origin: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub transform_3d_from_projection: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub projection_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub projection_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub projection_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub projection_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub projection_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub projection_from_transform_3d: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub projection_from_x_axis_y_axis_z_axis_w_axis: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub color_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub color_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub color_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub color_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub color_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub color_from_from_alpha: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub color_from_r_g_b: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub color_from_r_g_b_a: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub color_from_code: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub color_from_code_alpha: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub string_name_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub string_name_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub string_name_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub string_name_operator_less: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub string_name_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub string_name_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub string_name_from_string: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub string_name_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub node_path_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub node_path_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub node_path_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub node_path_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub node_path_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub node_path_from_string: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub node_path_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub rid_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub rid_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub rid_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub rid_operator_less: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub rid_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub rid_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub object_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub object_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub callable_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub callable_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub callable_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub callable_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub callable_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub callable_from_object_method: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub callable_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub signal_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub signal_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub signal_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub signal_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub signal_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub signal_from_object_signal: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub signal_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub dictionary_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub dictionary_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub dictionary_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub dictionary_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub dictionary_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub dictionary_from_base_key_type_key_class_name_key_script_value_type_value_class_name_value_script: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub dictionary_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub array_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub array_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub array_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub array_operator_less: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub array_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub array_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub array_from_base_type_class_name_script: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub array_from_packed_byte_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub array_from_packed_int32_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub array_from_packed_int64_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub array_from_packed_float32_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub array_from_packed_float64_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub array_from_packed_string_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub array_from_packed_vector2_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub array_from_packed_vector3_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub array_from_packed_color_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub array_from_packed_vector4_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub array_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub packed_byte_array_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub packed_byte_array_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub packed_byte_array_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub packed_byte_array_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_byte_array_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_byte_array_from_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_byte_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub packed_int32_array_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub packed_int32_array_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub packed_int32_array_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub packed_int32_array_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_int32_array_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_int32_array_from_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_int32_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub packed_int64_array_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub packed_int64_array_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub packed_int64_array_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub packed_int64_array_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_int64_array_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_int64_array_from_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_int64_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub packed_float32_array_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub packed_float32_array_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub packed_float32_array_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub packed_float32_array_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_float32_array_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_float32_array_from_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_float32_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub packed_float64_array_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub packed_float64_array_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub packed_float64_array_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub packed_float64_array_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_float64_array_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_float64_array_from_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_float64_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub packed_string_array_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub packed_string_array_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub packed_string_array_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub packed_string_array_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_string_array_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_string_array_from_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_string_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub packed_vector2_array_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub packed_vector2_array_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub packed_vector2_array_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub packed_vector2_array_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_vector2_array_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_vector2_array_from_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_vector2_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub packed_vector3_array_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub packed_vector3_array_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub packed_vector3_array_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub packed_vector3_array_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_vector3_array_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_vector3_array_from_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_vector3_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub packed_color_array_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub packed_color_array_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub packed_color_array_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub packed_color_array_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_color_array_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_color_array_from_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_color_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr), pub packed_vector4_array_to_variant: unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr), pub packed_vector4_array_from_variant: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr), pub packed_vector4_array_operator_equal: unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr), pub packed_vector4_array_construct_default: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_vector4_array_construct_copy: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_vector4_array_from_array: unsafe extern "C" fn(GDExtensionUninitializedTypePtr, * const GDExtensionConstTypePtr), pub packed_vector4_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
}
impl BuiltinLifecycleTable {
    pub const CLASS_COUNT: usize = 38usize;
    pub const METHOD_COUNT: usize = 0usize;
    #[doc = r" # Safety"]
    #[doc = r" - Must be called exactly once during library initialization."]
    #[doc = r" - All parameters (dependencies) must have been initialized and valid."]
    pub unsafe fn load(interface: &crate::GDExtensionInterface,) -> Self {
        let get_construct_fn = interface.variant_get_ptr_constructor.unwrap();
        let get_destroy_fn = interface.variant_get_ptr_destructor.unwrap();
        let get_operator_fn = interface.variant_get_ptr_operator_evaluator.unwrap();
        let get_to_variant_fn = interface.get_variant_from_type_constructor.unwrap();
        let get_from_variant_fn = interface.get_variant_to_type_constructor.unwrap();
        Self {
            bool_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_BOOL)
                };
                crate::validate_builtin_lifecycle(fptr, "bool_to_variant")
            },
            bool_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_BOOL)
                };
                crate::validate_builtin_lifecycle(fptr, "bool_from_variant")
            },
            int_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_INT)
                };
                crate::validate_builtin_lifecycle(fptr, "int_to_variant")
            },
            int_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_INT)
                };
                crate::validate_builtin_lifecycle(fptr, "int_from_variant")
            },
            float_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_FLOAT)
                };
                crate::validate_builtin_lifecycle(fptr, "float_to_variant")
            },
            float_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_FLOAT)
                };
                crate::validate_builtin_lifecycle(fptr, "float_from_variant")
            },
            string_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING)
                };
                crate::validate_builtin_lifecycle(fptr, "string_to_variant")
            },
            string_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING)
                };
                crate::validate_builtin_lifecycle(fptr, "string_from_variant")
            },
            string_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_STRING, crate::GDEXTENSION_VARIANT_TYPE_STRING)
                };
                crate::validate_builtin_lifecycle(fptr, "string_operator_equal")
            },
            string_operator_less: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_LESS, crate::GDEXTENSION_VARIANT_TYPE_STRING, crate::GDEXTENSION_VARIANT_TYPE_STRING)
                };
                crate::validate_builtin_lifecycle(fptr, "string_operator_less")
            },
            string_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "string_construct_default")
            },
            string_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "string_construct_copy")
            },
            string_from_string_name: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "string_from_string_name")
            },
            string_from_node_path: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "string_from_node_path")
            },
            string_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING)
                };
                crate::validate_builtin_lifecycle(fptr, "string_destroy")
            },
            vector2_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2_to_variant")
            },
            vector2_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2_from_variant")
            },
            vector2_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2_operator_equal")
            },
            vector2_operator_less: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_LESS, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2_operator_less")
            },
            vector2_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2_construct_default")
            },
            vector2_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2_construct_copy")
            },
            vector2_from_vector2i: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2_from_vector2i")
            },
            vector2_from_x_y: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2_from_x_y")
            },
            vector2i_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2i_to_variant")
            },
            vector2i_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2i_from_variant")
            },
            vector2i_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2i_operator_equal")
            },
            vector2i_operator_less: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_LESS, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2i_operator_less")
            },
            vector2i_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2i_construct_default")
            },
            vector2i_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2i_construct_copy")
            },
            vector2i_from_vector2: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2i_from_vector2")
            },
            vector2i_from_x_y: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector2i_from_x_y")
            },
            rect2_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2_to_variant")
            },
            rect2_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2_from_variant")
            },
            rect2_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_RECT2, crate::GDEXTENSION_VARIANT_TYPE_RECT2)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2_operator_equal")
            },
            rect2_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2_construct_default")
            },
            rect2_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2_construct_copy")
            },
            rect2_from_rect2i: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2_from_rect2i")
            },
            rect2_from_position_size: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2_from_position_size")
            },
            rect2_from_x_y_width_height: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2, 4i32)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2_from_x_y_width_height")
            },
            rect2i_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2i_to_variant")
            },
            rect2i_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2i_from_variant")
            },
            rect2i_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_RECT2I, crate::GDEXTENSION_VARIANT_TYPE_RECT2I)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2i_operator_equal")
            },
            rect2i_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2i_construct_default")
            },
            rect2i_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2i_construct_copy")
            },
            rect2i_from_rect2: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2i_from_rect2")
            },
            rect2i_from_position_size: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2i_from_position_size")
            },
            rect2i_from_x_y_width_height: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I, 4i32)
                };
                crate::validate_builtin_lifecycle(fptr, "rect2i_from_x_y_width_height")
            },
            vector3_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3_to_variant")
            },
            vector3_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3_from_variant")
            },
            vector3_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3_operator_equal")
            },
            vector3_operator_less: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_LESS, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3_operator_less")
            },
            vector3_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3_construct_default")
            },
            vector3_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3_construct_copy")
            },
            vector3_from_vector3i: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3_from_vector3i")
            },
            vector3_from_x_y_z: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3_from_x_y_z")
            },
            vector3i_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3i_to_variant")
            },
            vector3i_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3i_from_variant")
            },
            vector3i_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3i_operator_equal")
            },
            vector3i_operator_less: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_LESS, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3i_operator_less")
            },
            vector3i_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3i_construct_default")
            },
            vector3i_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3i_construct_copy")
            },
            vector3i_from_vector3: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3i_from_vector3")
            },
            vector3i_from_x_y_z: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector3i_from_x_y_z")
            },
            transform_2d_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_2d_to_variant")
            },
            transform_2d_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_2d_from_variant")
            },
            transform_2d_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_2d_operator_equal")
            },
            transform_2d_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_2d_construct_default")
            },
            transform_2d_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_2d_construct_copy")
            },
            transform_2d_from_rotation_position: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_2d_from_rotation_position")
            },
            transform_2d_from_rotation_scale_skew_position: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_2d_from_rotation_scale_skew_position")
            },
            transform_2d_from_x_axis_y_axis_origin: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, 4i32)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_2d_from_x_axis_y_axis_origin")
            },
            vector4_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4_to_variant")
            },
            vector4_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4_from_variant")
            },
            vector4_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4_operator_equal")
            },
            vector4_operator_less: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_LESS, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4_operator_less")
            },
            vector4_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4_construct_default")
            },
            vector4_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4_construct_copy")
            },
            vector4_from_vector4i: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4_from_vector4i")
            },
            vector4_from_x_y_z_w: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4_from_x_y_z_w")
            },
            vector4i_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4i_to_variant")
            },
            vector4i_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4i_from_variant")
            },
            vector4i_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4i_operator_equal")
            },
            vector4i_operator_less: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_LESS, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4i_operator_less")
            },
            vector4i_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4i_construct_default")
            },
            vector4i_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4i_construct_copy")
            },
            vector4i_from_vector4: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4i_from_vector4")
            },
            vector4i_from_x_y_z_w: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "vector4i_from_x_y_z_w")
            },
            plane_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE)
                };
                crate::validate_builtin_lifecycle(fptr, "plane_to_variant")
            },
            plane_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE)
                };
                crate::validate_builtin_lifecycle(fptr, "plane_from_variant")
            },
            plane_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_PLANE, crate::GDEXTENSION_VARIANT_TYPE_PLANE)
                };
                crate::validate_builtin_lifecycle(fptr, "plane_operator_equal")
            },
            plane_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "plane_construct_default")
            },
            plane_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "plane_construct_copy")
            },
            plane_from_normal: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "plane_from_normal")
            },
            plane_from_normal_d: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "plane_from_normal_d")
            },
            plane_from_normal_point: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 4i32)
                };
                crate::validate_builtin_lifecycle(fptr, "plane_from_normal_point")
            },
            plane_from_point1_point2_point3: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 5i32)
                };
                crate::validate_builtin_lifecycle(fptr, "plane_from_point1_point2_point3")
            },
            plane_from_a_b_c_d: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 6i32)
                };
                crate::validate_builtin_lifecycle(fptr, "plane_from_a_b_c_d")
            },
            quaternion_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION)
                };
                crate::validate_builtin_lifecycle(fptr, "quaternion_to_variant")
            },
            quaternion_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION)
                };
                crate::validate_builtin_lifecycle(fptr, "quaternion_from_variant")
            },
            quaternion_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, crate::GDEXTENSION_VARIANT_TYPE_QUATERNION)
                };
                crate::validate_builtin_lifecycle(fptr, "quaternion_operator_equal")
            },
            quaternion_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "quaternion_construct_default")
            },
            quaternion_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "quaternion_construct_copy")
            },
            quaternion_from_basis: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "quaternion_from_basis")
            },
            quaternion_from_axis_angle: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "quaternion_from_axis_angle")
            },
            quaternion_from_arc_from_arc_to: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, 4i32)
                };
                crate::validate_builtin_lifecycle(fptr, "quaternion_from_arc_from_arc_to")
            },
            quaternion_from_x_y_z_w: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, 5i32)
                };
                crate::validate_builtin_lifecycle(fptr, "quaternion_from_x_y_z_w")
            },
            aabb_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_AABB)
                };
                crate::validate_builtin_lifecycle(fptr, "aabb_to_variant")
            },
            aabb_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_AABB)
                };
                crate::validate_builtin_lifecycle(fptr, "aabb_from_variant")
            },
            aabb_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_AABB, crate::GDEXTENSION_VARIANT_TYPE_AABB)
                };
                crate::validate_builtin_lifecycle(fptr, "aabb_operator_equal")
            },
            aabb_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_AABB, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "aabb_construct_default")
            },
            aabb_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_AABB, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "aabb_construct_copy")
            },
            aabb_from_position_size: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_AABB, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "aabb_from_position_size")
            },
            basis_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS)
                };
                crate::validate_builtin_lifecycle(fptr, "basis_to_variant")
            },
            basis_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS)
                };
                crate::validate_builtin_lifecycle(fptr, "basis_from_variant")
            },
            basis_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_BASIS, crate::GDEXTENSION_VARIANT_TYPE_BASIS)
                };
                crate::validate_builtin_lifecycle(fptr, "basis_operator_equal")
            },
            basis_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "basis_construct_default")
            },
            basis_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "basis_construct_copy")
            },
            basis_from_quaternion: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "basis_from_quaternion")
            },
            basis_from_axis_angle: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "basis_from_axis_angle")
            },
            basis_from_x_axis_y_axis_z_axis: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS, 4i32)
                };
                crate::validate_builtin_lifecycle(fptr, "basis_from_x_axis_y_axis_z_axis")
            },
            transform_3d_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_3d_to_variant")
            },
            transform_3d_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_3d_from_variant")
            },
            transform_3d_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_3d_operator_equal")
            },
            transform_3d_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_3d_construct_default")
            },
            transform_3d_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_3d_construct_copy")
            },
            transform_3d_from_basis_origin: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_3d_from_basis_origin")
            },
            transform_3d_from_x_axis_y_axis_z_axis_origin: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_3d_from_x_axis_y_axis_z_axis_origin")
            },
            transform_3d_from_projection: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, 4i32)
                };
                crate::validate_builtin_lifecycle(fptr, "transform_3d_from_projection")
            },
            projection_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PROJECTION)
                };
                crate::validate_builtin_lifecycle(fptr, "projection_to_variant")
            },
            projection_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PROJECTION)
                };
                crate::validate_builtin_lifecycle(fptr, "projection_from_variant")
            },
            projection_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, crate::GDEXTENSION_VARIANT_TYPE_PROJECTION)
                };
                crate::validate_builtin_lifecycle(fptr, "projection_operator_equal")
            },
            projection_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "projection_construct_default")
            },
            projection_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "projection_construct_copy")
            },
            projection_from_transform_3d: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "projection_from_transform_3d")
            },
            projection_from_x_axis_y_axis_z_axis_w_axis: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "projection_from_x_axis_y_axis_z_axis_w_axis")
            },
            color_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR)
                };
                crate::validate_builtin_lifecycle(fptr, "color_to_variant")
            },
            color_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR)
                };
                crate::validate_builtin_lifecycle(fptr, "color_from_variant")
            },
            color_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_COLOR, crate::GDEXTENSION_VARIANT_TYPE_COLOR)
                };
                crate::validate_builtin_lifecycle(fptr, "color_operator_equal")
            },
            color_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "color_construct_default")
            },
            color_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "color_construct_copy")
            },
            color_from_from_alpha: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "color_from_from_alpha")
            },
            color_from_r_g_b: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "color_from_r_g_b")
            },
            color_from_r_g_b_a: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 4i32)
                };
                crate::validate_builtin_lifecycle(fptr, "color_from_r_g_b_a")
            },
            color_from_code: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 5i32)
                };
                crate::validate_builtin_lifecycle(fptr, "color_from_code")
            },
            color_from_code_alpha: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 6i32)
                };
                crate::validate_builtin_lifecycle(fptr, "color_from_code_alpha")
            },
            string_name_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME)
                };
                crate::validate_builtin_lifecycle(fptr, "string_name_to_variant")
            },
            string_name_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME)
                };
                crate::validate_builtin_lifecycle(fptr, "string_name_from_variant")
            },
            string_name_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME)
                };
                crate::validate_builtin_lifecycle(fptr, "string_name_operator_equal")
            },
            string_name_operator_less: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_LESS, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME)
                };
                crate::validate_builtin_lifecycle(fptr, "string_name_operator_less")
            },
            string_name_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "string_name_construct_default")
            },
            string_name_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "string_name_construct_copy")
            },
            string_name_from_string: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "string_name_from_string")
            },
            string_name_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME)
                };
                crate::validate_builtin_lifecycle(fptr, "string_name_destroy")
            },
            node_path_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH)
                };
                crate::validate_builtin_lifecycle(fptr, "node_path_to_variant")
            },
            node_path_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH)
                };
                crate::validate_builtin_lifecycle(fptr, "node_path_from_variant")
            },
            node_path_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH)
                };
                crate::validate_builtin_lifecycle(fptr, "node_path_operator_equal")
            },
            node_path_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "node_path_construct_default")
            },
            node_path_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "node_path_construct_copy")
            },
            node_path_from_string: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "node_path_from_string")
            },
            node_path_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH)
                };
                crate::validate_builtin_lifecycle(fptr, "node_path_destroy")
            },
            rid_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_RID)
                };
                crate::validate_builtin_lifecycle(fptr, "rid_to_variant")
            },
            rid_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_RID)
                };
                crate::validate_builtin_lifecycle(fptr, "rid_from_variant")
            },
            rid_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_RID, crate::GDEXTENSION_VARIANT_TYPE_RID)
                };
                crate::validate_builtin_lifecycle(fptr, "rid_operator_equal")
            },
            rid_operator_less: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_LESS, crate::GDEXTENSION_VARIANT_TYPE_RID, crate::GDEXTENSION_VARIANT_TYPE_RID)
                };
                crate::validate_builtin_lifecycle(fptr, "rid_operator_less")
            },
            rid_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_RID, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "rid_construct_default")
            },
            rid_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_RID, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "rid_construct_copy")
            },
            object_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_OBJECT)
                };
                crate::validate_builtin_lifecycle(fptr, "object_to_variant")
            },
            object_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_OBJECT)
                };
                crate::validate_builtin_lifecycle(fptr, "object_from_variant")
            },
            callable_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_CALLABLE)
                };
                crate::validate_builtin_lifecycle(fptr, "callable_to_variant")
            },
            callable_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_CALLABLE)
                };
                crate::validate_builtin_lifecycle(fptr, "callable_from_variant")
            },
            callable_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, crate::GDEXTENSION_VARIANT_TYPE_CALLABLE)
                };
                crate::validate_builtin_lifecycle(fptr, "callable_operator_equal")
            },
            callable_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "callable_construct_default")
            },
            callable_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "callable_construct_copy")
            },
            callable_from_object_method: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "callable_from_object_method")
            },
            callable_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_CALLABLE)
                };
                crate::validate_builtin_lifecycle(fptr, "callable_destroy")
            },
            signal_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_SIGNAL)
                };
                crate::validate_builtin_lifecycle(fptr, "signal_to_variant")
            },
            signal_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_SIGNAL)
                };
                crate::validate_builtin_lifecycle(fptr, "signal_from_variant")
            },
            signal_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, crate::GDEXTENSION_VARIANT_TYPE_SIGNAL)
                };
                crate::validate_builtin_lifecycle(fptr, "signal_operator_equal")
            },
            signal_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "signal_construct_default")
            },
            signal_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "signal_construct_copy")
            },
            signal_from_object_signal: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "signal_from_object_signal")
            },
            signal_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_SIGNAL)
                };
                crate::validate_builtin_lifecycle(fptr, "signal_destroy")
            },
            dictionary_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY)
                };
                crate::validate_builtin_lifecycle(fptr, "dictionary_to_variant")
            },
            dictionary_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY)
                };
                crate::validate_builtin_lifecycle(fptr, "dictionary_from_variant")
            },
            dictionary_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY)
                };
                crate::validate_builtin_lifecycle(fptr, "dictionary_operator_equal")
            },
            dictionary_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "dictionary_construct_default")
            },
            dictionary_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "dictionary_construct_copy")
            },
            dictionary_from_base_key_type_key_class_name_key_script_value_type_value_class_name_value_script: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "dictionary_from_base_key_type_key_class_name_key_script_value_type_value_class_name_value_script")
            },
            dictionary_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY)
                };
                crate::validate_builtin_lifecycle(fptr, "dictionary_destroy")
            },
            array_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "array_to_variant")
            },
            array_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "array_from_variant")
            },
            array_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, crate::GDEXTENSION_VARIANT_TYPE_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "array_operator_equal")
            },
            array_operator_less: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_LESS, crate::GDEXTENSION_VARIANT_TYPE_ARRAY, crate::GDEXTENSION_VARIANT_TYPE_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "array_operator_less")
            },
            array_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "array_construct_default")
            },
            array_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "array_construct_copy")
            },
            array_from_base_type_class_name_script: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "array_from_base_type_class_name_script")
            },
            array_from_packed_byte_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 3i32)
                };
                crate::validate_builtin_lifecycle(fptr, "array_from_packed_byte_array")
            },
            array_from_packed_int32_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 4i32)
                };
                crate::validate_builtin_lifecycle(fptr, "array_from_packed_int32_array")
            },
            array_from_packed_int64_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 5i32)
                };
                crate::validate_builtin_lifecycle(fptr, "array_from_packed_int64_array")
            },
            array_from_packed_float32_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 6i32)
                };
                crate::validate_builtin_lifecycle(fptr, "array_from_packed_float32_array")
            },
            array_from_packed_float64_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 7i32)
                };
                crate::validate_builtin_lifecycle(fptr, "array_from_packed_float64_array")
            },
            array_from_packed_string_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 8i32)
                };
                crate::validate_builtin_lifecycle(fptr, "array_from_packed_string_array")
            },
            array_from_packed_vector2_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 9i32)
                };
                crate::validate_builtin_lifecycle(fptr, "array_from_packed_vector2_array")
            },
            array_from_packed_vector3_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 10i32)
                };
                crate::validate_builtin_lifecycle(fptr, "array_from_packed_vector3_array")
            },
            array_from_packed_color_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 11i32)
                };
                crate::validate_builtin_lifecycle(fptr, "array_from_packed_color_array")
            },
            array_from_packed_vector4_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 12i32)
                };
                crate::validate_builtin_lifecycle(fptr, "array_from_packed_vector4_array")
            },
            array_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "array_destroy")
            },
            packed_byte_array_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_byte_array_to_variant")
            },
            packed_byte_array_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_byte_array_from_variant")
            },
            packed_byte_array_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_byte_array_operator_equal")
            },
            packed_byte_array_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_byte_array_construct_default")
            },
            packed_byte_array_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_byte_array_construct_copy")
            },
            packed_byte_array_from_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_byte_array_from_array")
            },
            packed_byte_array_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_byte_array_destroy")
            },
            packed_int32_array_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int32_array_to_variant")
            },
            packed_int32_array_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int32_array_from_variant")
            },
            packed_int32_array_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int32_array_operator_equal")
            },
            packed_int32_array_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int32_array_construct_default")
            },
            packed_int32_array_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int32_array_construct_copy")
            },
            packed_int32_array_from_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int32_array_from_array")
            },
            packed_int32_array_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int32_array_destroy")
            },
            packed_int64_array_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int64_array_to_variant")
            },
            packed_int64_array_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int64_array_from_variant")
            },
            packed_int64_array_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int64_array_operator_equal")
            },
            packed_int64_array_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int64_array_construct_default")
            },
            packed_int64_array_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int64_array_construct_copy")
            },
            packed_int64_array_from_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int64_array_from_array")
            },
            packed_int64_array_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_int64_array_destroy")
            },
            packed_float32_array_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float32_array_to_variant")
            },
            packed_float32_array_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float32_array_from_variant")
            },
            packed_float32_array_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float32_array_operator_equal")
            },
            packed_float32_array_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float32_array_construct_default")
            },
            packed_float32_array_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float32_array_construct_copy")
            },
            packed_float32_array_from_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float32_array_from_array")
            },
            packed_float32_array_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float32_array_destroy")
            },
            packed_float64_array_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float64_array_to_variant")
            },
            packed_float64_array_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float64_array_from_variant")
            },
            packed_float64_array_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float64_array_operator_equal")
            },
            packed_float64_array_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float64_array_construct_default")
            },
            packed_float64_array_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float64_array_construct_copy")
            },
            packed_float64_array_from_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float64_array_from_array")
            },
            packed_float64_array_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_float64_array_destroy")
            },
            packed_string_array_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_string_array_to_variant")
            },
            packed_string_array_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_string_array_from_variant")
            },
            packed_string_array_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_string_array_operator_equal")
            },
            packed_string_array_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_string_array_construct_default")
            },
            packed_string_array_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_string_array_construct_copy")
            },
            packed_string_array_from_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_string_array_from_array")
            },
            packed_string_array_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_string_array_destroy")
            },
            packed_vector2_array_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector2_array_to_variant")
            },
            packed_vector2_array_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector2_array_from_variant")
            },
            packed_vector2_array_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector2_array_operator_equal")
            },
            packed_vector2_array_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector2_array_construct_default")
            },
            packed_vector2_array_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector2_array_construct_copy")
            },
            packed_vector2_array_from_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector2_array_from_array")
            },
            packed_vector2_array_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector2_array_destroy")
            },
            packed_vector3_array_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector3_array_to_variant")
            },
            packed_vector3_array_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector3_array_from_variant")
            },
            packed_vector3_array_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector3_array_operator_equal")
            },
            packed_vector3_array_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector3_array_construct_default")
            },
            packed_vector3_array_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector3_array_construct_copy")
            },
            packed_vector3_array_from_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector3_array_from_array")
            },
            packed_vector3_array_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector3_array_destroy")
            },
            packed_color_array_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_color_array_to_variant")
            },
            packed_color_array_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_color_array_from_variant")
            },
            packed_color_array_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_color_array_operator_equal")
            },
            packed_color_array_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_color_array_construct_default")
            },
            packed_color_array_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_color_array_construct_copy")
            },
            packed_color_array_from_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_color_array_from_array")
            },
            packed_color_array_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_color_array_destroy")
            },
            packed_vector4_array_to_variant: {
                let fptr = unsafe {
                    get_to_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector4_array_to_variant")
            },
            packed_vector4_array_from_variant: {
                let fptr = unsafe {
                    get_from_variant_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector4_array_from_variant")
            },
            packed_vector4_array_operator_equal: {
                let fptr = unsafe {
                    get_operator_fn(crate::GDEXTENSION_VARIANT_OP_EQUAL, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector4_array_operator_equal")
            },
            packed_vector4_array_construct_default: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, 0i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector4_array_construct_default")
            },
            packed_vector4_array_construct_copy: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, 1i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector4_array_construct_copy")
            },
            packed_vector4_array_from_array: {
                let fptr = unsafe {
                    get_construct_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY, 2i32)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector4_array_from_array")
            },
            packed_vector4_array_destroy: {
                let fptr = unsafe {
                    get_destroy_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY)
                };
                crate::validate_builtin_lifecycle(fptr, "packed_vector4_array_destroy")
            },
            
        }
    }
}