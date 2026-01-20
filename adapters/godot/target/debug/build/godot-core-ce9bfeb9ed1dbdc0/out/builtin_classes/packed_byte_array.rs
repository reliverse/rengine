use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, ClassId, CowArg, InParamTuple, OutParamTuple, ParamTuple, RefArg, Signature
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
#[repr(transparent)]
pub struct InnerPackedByteArray < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedByteArray < 'a > {
    pub fn from_outer(outer: &PackedByteArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn get(&self, index: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(722usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(723usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "set", self.sys_ptr, args)
        }
    }
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(724usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(725usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(726usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(727usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &PackedByteArray,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(728usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(729usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(730usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(731usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(732usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(733usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(734usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(735usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = (i64, i64,);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(736usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "slice", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(737usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: i64, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, bool,);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(738usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(739usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: i64, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(740usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: i64, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(741usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(742usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "count", self.sys_ptr, args)
        }
    }
    pub fn erase(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(743usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "erase", self.sys_ptr, args)
        }
    }
    pub fn get_string_from_multibyte_char(&self, encoding: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (encoding.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(749usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_multibyte_char", self.sys_ptr, args)
        }
    }
    pub fn compress(&self, compression_mode: i64,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = (i64,);
        let args = (compression_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(751usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "compress", self.sys_ptr, args)
        }
    }
    pub fn decompress(&self, buffer_size: i64, compression_mode: i64,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = (i64, i64,);
        let args = (buffer_size, compression_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(752usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decompress", self.sys_ptr, args)
        }
    }
    pub fn decompress_dynamic(&self, max_output_size: i64, compression_mode: i64,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = (i64, i64,);
        let args = (max_output_size, compression_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(753usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decompress_dynamic", self.sys_ptr, args)
        }
    }
    pub fn decode_u8(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(754usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_u8", self.sys_ptr, args)
        }
    }
    pub fn decode_s8(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(755usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_s8", self.sys_ptr, args)
        }
    }
    pub fn decode_u16(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(756usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_u16", self.sys_ptr, args)
        }
    }
    pub fn decode_s16(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(757usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_s16", self.sys_ptr, args)
        }
    }
    pub fn decode_u32(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(758usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_u32", self.sys_ptr, args)
        }
    }
    pub fn decode_s32(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(759usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_s32", self.sys_ptr, args)
        }
    }
    pub fn decode_u64(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(760usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_u64", self.sys_ptr, args)
        }
    }
    pub fn decode_s64(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(761usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_s64", self.sys_ptr, args)
        }
    }
    pub fn decode_half(&self, byte_offset: i64,) -> f64 {
        type CallRet = f64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(762usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_half", self.sys_ptr, args)
        }
    }
    pub fn decode_float(&self, byte_offset: i64,) -> f64 {
        type CallRet = f64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(763usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_float", self.sys_ptr, args)
        }
    }
    pub fn decode_double(&self, byte_offset: i64,) -> f64 {
        type CallRet = f64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(764usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_double", self.sys_ptr, args)
        }
    }
    pub fn has_encoded_var(&self, byte_offset: i64, allow_objects: bool,) -> bool {
        type CallRet = bool;
        type CallParams = (i64, bool,);
        let args = (byte_offset, allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(765usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "has_encoded_var", self.sys_ptr, args)
        }
    }
    pub fn decode_var(&self, byte_offset: i64, allow_objects: bool,) -> Variant {
        type CallRet = Variant;
        type CallParams = (i64, bool,);
        let args = (byte_offset, allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(766usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_var", self.sys_ptr, args)
        }
    }
    pub fn decode_var_size(&self, byte_offset: i64, allow_objects: bool,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, bool,);
        let args = (byte_offset, allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(767usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_var_size", self.sys_ptr, args)
        }
    }
    pub fn to_int32_array(&self,) -> PackedInt32Array {
        type CallRet = PackedInt32Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(768usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_int32_array", self.sys_ptr, args)
        }
    }
    pub fn to_int64_array(&self,) -> PackedInt64Array {
        type CallRet = PackedInt64Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(769usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_int64_array", self.sys_ptr, args)
        }
    }
    pub fn to_float32_array(&self,) -> PackedFloat32Array {
        type CallRet = PackedFloat32Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(770usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_float32_array", self.sys_ptr, args)
        }
    }
    pub fn to_float64_array(&self,) -> PackedFloat64Array {
        type CallRet = PackedFloat64Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(771usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_float64_array", self.sys_ptr, args)
        }
    }
    pub fn to_vector2_array(&self,) -> PackedVector2Array {
        type CallRet = PackedVector2Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(772usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_vector2_array", self.sys_ptr, args)
        }
    }
    pub fn to_vector3_array(&self,) -> PackedVector3Array {
        type CallRet = PackedVector3Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(773usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_vector3_array", self.sys_ptr, args)
        }
    }
    pub fn to_vector4_array(&self,) -> PackedVector4Array {
        type CallRet = PackedVector4Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(774usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_vector4_array", self.sys_ptr, args)
        }
    }
    pub fn to_color_array(&self,) -> PackedColorArray {
        type CallRet = PackedColorArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(775usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_color_array", self.sys_ptr, args)
        }
    }
    pub fn bswap16(&mut self, offset: i64, count: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (offset, count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(776usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "bswap16", self.sys_ptr, args)
        }
    }
    pub fn bswap32(&mut self, offset: i64, count: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (offset, count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(777usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "bswap32", self.sys_ptr, args)
        }
    }
    pub fn bswap64(&mut self, offset: i64, count: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (offset, count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(778usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "bswap64", self.sys_ptr, args)
        }
    }
    pub fn encode_u8(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(779usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_u8", self.sys_ptr, args)
        }
    }
    pub fn encode_s8(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(780usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_s8", self.sys_ptr, args)
        }
    }
    pub fn encode_u16(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(781usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_u16", self.sys_ptr, args)
        }
    }
    pub fn encode_s16(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(782usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_s16", self.sys_ptr, args)
        }
    }
    pub fn encode_u32(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(783usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_u32", self.sys_ptr, args)
        }
    }
    pub fn encode_s32(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(784usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_s32", self.sys_ptr, args)
        }
    }
    pub fn encode_u64(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(785usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_u64", self.sys_ptr, args)
        }
    }
    pub fn encode_s64(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(786usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_s64", self.sys_ptr, args)
        }
    }
    pub fn encode_half(&mut self, byte_offset: i64, value: f64,) {
        type CallRet = ();
        type CallParams = (i64, f64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(787usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_half", self.sys_ptr, args)
        }
    }
    pub fn encode_float(&mut self, byte_offset: i64, value: f64,) {
        type CallRet = ();
        type CallParams = (i64, f64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(788usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_float", self.sys_ptr, args)
        }
    }
    pub fn encode_double(&mut self, byte_offset: i64, value: f64,) {
        type CallRet = ();
        type CallParams = (i64, f64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(789usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_double", self.sys_ptr, args)
        }
    }
    pub fn encode_var(&mut self, byte_offset: i64, value: &Variant, allow_objects: bool,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (i64, RefArg < 'a0, Variant >, bool,);
        let args = (byte_offset, RefArg::new(value), allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(790usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_var", self.sys_ptr, args)
        }
    }
}
impl PackedByteArray {
    pub fn get_string_from_ascii(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(744usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_ascii", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_string_from_utf8(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(745usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_utf8", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_string_from_utf16(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(746usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_utf16", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_string_from_utf32(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(747usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_utf32", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_string_from_wchar(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(748usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_wchar", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn hex_encode(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(750usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "hex_encode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
}