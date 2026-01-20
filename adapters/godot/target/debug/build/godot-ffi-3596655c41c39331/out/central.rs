#[cfg(target_pointer_width = "32")]
#[cfg_attr(published_docs, doc(cfg(target_pointer_width = "32")))]
pub mod types {
    pub type OpaqueNil = crate::opaque::Opaque < 0usize >;
    pub type OpaqueBool = crate::opaque::Opaque < 1usize >;
    pub type OpaqueInt = crate::opaque::Opaque < 8usize >;
    pub type OpaqueFloat = crate::opaque::Opaque < 8usize >;
    pub type OpaqueString = crate::opaque::Opaque < 4usize >;
    pub type OpaqueVector2 = crate::opaque::Opaque < 8usize >;
    pub type OpaqueVector2i = crate::opaque::Opaque < 8usize >;
    pub type OpaqueRect2 = crate::opaque::Opaque < 16usize >;
    pub type OpaqueRect2i = crate::opaque::Opaque < 16usize >;
    pub type OpaqueVector3 = crate::opaque::Opaque < 12usize >;
    pub type OpaqueVector3i = crate::opaque::Opaque < 12usize >;
    pub type OpaqueTransform2D = crate::opaque::Opaque < 24usize >;
    pub type OpaqueVector4 = crate::opaque::Opaque < 16usize >;
    pub type OpaqueVector4i = crate::opaque::Opaque < 16usize >;
    pub type OpaquePlane = crate::opaque::Opaque < 16usize >;
    pub type OpaqueQuaternion = crate::opaque::Opaque < 16usize >;
    pub type OpaqueAabb = crate::opaque::Opaque < 24usize >;
    pub type OpaqueBasis = crate::opaque::Opaque < 36usize >;
    pub type OpaqueTransform3D = crate::opaque::Opaque < 48usize >;
    pub type OpaqueProjection = crate::opaque::Opaque < 64usize >;
    pub type OpaqueColor = crate::opaque::Opaque < 16usize >;
    pub type OpaqueStringName = crate::opaque::Opaque < 4usize >;
    pub type OpaqueNodePath = crate::opaque::Opaque < 4usize >;
    pub type OpaqueRid = crate::opaque::Opaque < 8usize >;
    pub type OpaqueObject = crate::opaque::Opaque < 4usize >;
    pub type OpaqueCallable = crate::opaque::Opaque < 16usize >;
    pub type OpaqueSignal = crate::opaque::Opaque < 16usize >;
    pub type OpaqueDictionary = crate::opaque::Opaque < 4usize >;
    pub type OpaqueArray = crate::opaque::Opaque < 4usize >;
    pub type OpaquePackedByteArray = crate::opaque::Opaque < 8usize >;
    pub type OpaquePackedInt32Array = crate::opaque::Opaque < 8usize >;
    pub type OpaquePackedInt64Array = crate::opaque::Opaque < 8usize >;
    pub type OpaquePackedFloat32Array = crate::opaque::Opaque < 8usize >;
    pub type OpaquePackedFloat64Array = crate::opaque::Opaque < 8usize >;
    pub type OpaquePackedStringArray = crate::opaque::Opaque < 8usize >;
    pub type OpaquePackedVector2Array = crate::opaque::Opaque < 8usize >;
    pub type OpaquePackedVector3Array = crate::opaque::Opaque < 8usize >;
    pub type OpaquePackedColorArray = crate::opaque::Opaque < 8usize >;
    pub type OpaquePackedVector4Array = crate::opaque::Opaque < 8usize >;
    pub type OpaqueVariant = crate::opaque::Opaque < 24usize >;
    
}
#[cfg(target_pointer_width = "64")]
#[cfg_attr(published_docs, doc(cfg(target_pointer_width = "64")))]
pub mod types {
    pub type OpaqueNil = crate::opaque::Opaque < 0usize >;
    pub type OpaqueBool = crate::opaque::Opaque < 1usize >;
    pub type OpaqueInt = crate::opaque::Opaque < 8usize >;
    pub type OpaqueFloat = crate::opaque::Opaque < 8usize >;
    pub type OpaqueString = crate::opaque::Opaque < 8usize >;
    pub type OpaqueVector2 = crate::opaque::Opaque < 8usize >;
    pub type OpaqueVector2i = crate::opaque::Opaque < 8usize >;
    pub type OpaqueRect2 = crate::opaque::Opaque < 16usize >;
    pub type OpaqueRect2i = crate::opaque::Opaque < 16usize >;
    pub type OpaqueVector3 = crate::opaque::Opaque < 12usize >;
    pub type OpaqueVector3i = crate::opaque::Opaque < 12usize >;
    pub type OpaqueTransform2D = crate::opaque::Opaque < 24usize >;
    pub type OpaqueVector4 = crate::opaque::Opaque < 16usize >;
    pub type OpaqueVector4i = crate::opaque::Opaque < 16usize >;
    pub type OpaquePlane = crate::opaque::Opaque < 16usize >;
    pub type OpaqueQuaternion = crate::opaque::Opaque < 16usize >;
    pub type OpaqueAabb = crate::opaque::Opaque < 24usize >;
    pub type OpaqueBasis = crate::opaque::Opaque < 36usize >;
    pub type OpaqueTransform3D = crate::opaque::Opaque < 48usize >;
    pub type OpaqueProjection = crate::opaque::Opaque < 64usize >;
    pub type OpaqueColor = crate::opaque::Opaque < 16usize >;
    pub type OpaqueStringName = crate::opaque::Opaque < 8usize >;
    pub type OpaqueNodePath = crate::opaque::Opaque < 8usize >;
    pub type OpaqueRid = crate::opaque::Opaque < 8usize >;
    pub type OpaqueObject = crate::opaque::Opaque < 8usize >;
    pub type OpaqueCallable = crate::opaque::Opaque < 16usize >;
    pub type OpaqueSignal = crate::opaque::Opaque < 16usize >;
    pub type OpaqueDictionary = crate::opaque::Opaque < 8usize >;
    pub type OpaqueArray = crate::opaque::Opaque < 8usize >;
    pub type OpaquePackedByteArray = crate::opaque::Opaque < 16usize >;
    pub type OpaquePackedInt32Array = crate::opaque::Opaque < 16usize >;
    pub type OpaquePackedInt64Array = crate::opaque::Opaque < 16usize >;
    pub type OpaquePackedFloat32Array = crate::opaque::Opaque < 16usize >;
    pub type OpaquePackedFloat64Array = crate::opaque::Opaque < 16usize >;
    pub type OpaquePackedStringArray = crate::opaque::Opaque < 16usize >;
    pub type OpaquePackedVector2Array = crate::opaque::Opaque < 16usize >;
    pub type OpaquePackedVector3Array = crate::opaque::Opaque < 16usize >;
    pub type OpaquePackedColorArray = crate::opaque::Opaque < 16usize >;
    pub type OpaquePackedVector4Array = crate::opaque::Opaque < 16usize >;
    pub type OpaqueVariant = crate::opaque::Opaque < 24usize >;
    
}
#[doc = r" Provides meta-information about the library and the Godot version in use."]
pub struct GdextBuild;
impl GdextBuild {
    #[doc = r" Godot version against which godot-rust was compiled."]
    #[doc = r""]
    #[doc = r" Example format: `v4.0.stable.official`"]
    pub const fn godot_static_version_string() -> &'static str {
        "v4.5.stable.official"
    }
    #[doc = r" Godot version against which godot-rust was compiled, as `(major, minor, patch)` triple."]
    pub const fn godot_static_version_triple() -> (u8, u8, u8) {
        (4u8, 5u8, 0u8)
    }
    #[doc = r" Version of the Godot engine which loaded godot-rust via GDExtension binding."]
    pub fn godot_runtime_version_string() -> String {
        let rt = unsafe {
            crate::runtime_metadata()
        };
        rt.version_string() . to_string()
    }
    #[doc = r" Version of the Godot engine which loaded godot-rust via GDExtension binding, as"]
    #[doc = r" `(major, minor, patch)` triple."]
    pub fn godot_runtime_version_triple() -> (u8, u8, u8) {
        let rt = unsafe {
            crate::runtime_metadata()
        };
        rt.version_triple()
    }
    #[doc = r#" For a string `"4.x"`, returns `true` if the current Godot version is strictly less than 4.x."#]
    #[doc = r""]
    #[doc = r#" Runtime equivalent of `#[cfg(before_api = "4.x")]`."#]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r" On bad input."]
    pub fn before_api(major_minor: &str) -> bool {
        let mut parts = major_minor.split('.');
        let queried_major = parts.next() . unwrap() . parse::< u8 > () . expect("invalid major version");
        let queried_minor = parts.next() . unwrap() . parse::< u8 > () . expect("invalid minor version");
        assert_eq !(queried_major, 4, "major version must be 4");
        let(_, minor, _) = Self::godot_runtime_version_triple();
        minor < queried_minor
    }
    #[doc = r#" For a string `"4.x"`, returns `true` if the current Godot version is equal or greater to 4.x."#]
    #[doc = r""]
    #[doc = r#" Runtime equivalent of `#[cfg(since_api = "4.x")]`."#]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r" On bad input."]
    pub fn since_api(major_minor: &str) -> bool {
        !Self::before_api(major_minor)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `Variant.Type`."]
pub struct VariantType {
    #[doc(hidden)]
    pub ord: i32
}
impl VariantType {
    #[doc(alias = "TYPE_NIL")]
    #[doc = "Godot enumerator name: `TYPE_NIL`"]
    pub const NIL: VariantType = VariantType {
        ord: 0i32
    };
    #[doc(alias = "TYPE_BOOL")]
    #[doc = "Godot enumerator name: `TYPE_BOOL`"]
    pub const BOOL: VariantType = VariantType {
        ord: 1i32
    };
    #[doc(alias = "TYPE_INT")]
    #[doc = "Godot enumerator name: `TYPE_INT`"]
    pub const INT: VariantType = VariantType {
        ord: 2i32
    };
    #[doc(alias = "TYPE_FLOAT")]
    #[doc = "Godot enumerator name: `TYPE_FLOAT`"]
    pub const FLOAT: VariantType = VariantType {
        ord: 3i32
    };
    #[doc(alias = "TYPE_STRING")]
    #[doc = "Godot enumerator name: `TYPE_STRING`"]
    pub const STRING: VariantType = VariantType {
        ord: 4i32
    };
    #[doc(alias = "TYPE_VECTOR2")]
    #[doc = "Godot enumerator name: `TYPE_VECTOR2`"]
    pub const VECTOR2: VariantType = VariantType {
        ord: 5i32
    };
    #[doc(alias = "TYPE_VECTOR2I")]
    #[doc = "Godot enumerator name: `TYPE_VECTOR2I`"]
    pub const VECTOR2I: VariantType = VariantType {
        ord: 6i32
    };
    #[doc(alias = "TYPE_RECT2")]
    #[doc = "Godot enumerator name: `TYPE_RECT2`"]
    pub const RECT2: VariantType = VariantType {
        ord: 7i32
    };
    #[doc(alias = "TYPE_RECT2I")]
    #[doc = "Godot enumerator name: `TYPE_RECT2I`"]
    pub const RECT2I: VariantType = VariantType {
        ord: 8i32
    };
    #[doc(alias = "TYPE_VECTOR3")]
    #[doc = "Godot enumerator name: `TYPE_VECTOR3`"]
    pub const VECTOR3: VariantType = VariantType {
        ord: 9i32
    };
    #[doc(alias = "TYPE_VECTOR3I")]
    #[doc = "Godot enumerator name: `TYPE_VECTOR3I`"]
    pub const VECTOR3I: VariantType = VariantType {
        ord: 10i32
    };
    #[doc(alias = "TYPE_TRANSFORM2D")]
    #[doc = "Godot enumerator name: `TYPE_TRANSFORM2D`"]
    pub const TRANSFORM2D: VariantType = VariantType {
        ord: 11i32
    };
    #[doc(alias = "TYPE_VECTOR4")]
    #[doc = "Godot enumerator name: `TYPE_VECTOR4`"]
    pub const VECTOR4: VariantType = VariantType {
        ord: 12i32
    };
    #[doc(alias = "TYPE_VECTOR4I")]
    #[doc = "Godot enumerator name: `TYPE_VECTOR4I`"]
    pub const VECTOR4I: VariantType = VariantType {
        ord: 13i32
    };
    #[doc(alias = "TYPE_PLANE")]
    #[doc = "Godot enumerator name: `TYPE_PLANE`"]
    pub const PLANE: VariantType = VariantType {
        ord: 14i32
    };
    #[doc(alias = "TYPE_QUATERNION")]
    #[doc = "Godot enumerator name: `TYPE_QUATERNION`"]
    pub const QUATERNION: VariantType = VariantType {
        ord: 15i32
    };
    #[doc(alias = "TYPE_AABB")]
    #[doc = "Godot enumerator name: `TYPE_AABB`"]
    pub const AABB: VariantType = VariantType {
        ord: 16i32
    };
    #[doc(alias = "TYPE_BASIS")]
    #[doc = "Godot enumerator name: `TYPE_BASIS`"]
    pub const BASIS: VariantType = VariantType {
        ord: 17i32
    };
    #[doc(alias = "TYPE_TRANSFORM3D")]
    #[doc = "Godot enumerator name: `TYPE_TRANSFORM3D`"]
    pub const TRANSFORM3D: VariantType = VariantType {
        ord: 18i32
    };
    #[doc(alias = "TYPE_PROJECTION")]
    #[doc = "Godot enumerator name: `TYPE_PROJECTION`"]
    pub const PROJECTION: VariantType = VariantType {
        ord: 19i32
    };
    #[doc(alias = "TYPE_COLOR")]
    #[doc = "Godot enumerator name: `TYPE_COLOR`"]
    pub const COLOR: VariantType = VariantType {
        ord: 20i32
    };
    #[doc(alias = "TYPE_STRING_NAME")]
    #[doc = "Godot enumerator name: `TYPE_STRING_NAME`"]
    pub const STRING_NAME: VariantType = VariantType {
        ord: 21i32
    };
    #[doc(alias = "TYPE_NODE_PATH")]
    #[doc = "Godot enumerator name: `TYPE_NODE_PATH`"]
    pub const NODE_PATH: VariantType = VariantType {
        ord: 22i32
    };
    #[doc(alias = "TYPE_RID")]
    #[doc = "Godot enumerator name: `TYPE_RID`"]
    pub const RID: VariantType = VariantType {
        ord: 23i32
    };
    #[doc(alias = "TYPE_OBJECT")]
    #[doc = "Godot enumerator name: `TYPE_OBJECT`"]
    pub const OBJECT: VariantType = VariantType {
        ord: 24i32
    };
    #[doc(alias = "TYPE_CALLABLE")]
    #[doc = "Godot enumerator name: `TYPE_CALLABLE`"]
    pub const CALLABLE: VariantType = VariantType {
        ord: 25i32
    };
    #[doc(alias = "TYPE_SIGNAL")]
    #[doc = "Godot enumerator name: `TYPE_SIGNAL`"]
    pub const SIGNAL: VariantType = VariantType {
        ord: 26i32
    };
    #[doc(alias = "TYPE_DICTIONARY")]
    #[doc = "Godot enumerator name: `TYPE_DICTIONARY`"]
    pub const DICTIONARY: VariantType = VariantType {
        ord: 27i32
    };
    #[doc(alias = "TYPE_ARRAY")]
    #[doc = "Godot enumerator name: `TYPE_ARRAY`"]
    pub const ARRAY: VariantType = VariantType {
        ord: 28i32
    };
    #[doc(alias = "TYPE_PACKED_BYTE_ARRAY")]
    #[doc = "Godot enumerator name: `TYPE_PACKED_BYTE_ARRAY`"]
    pub const PACKED_BYTE_ARRAY: VariantType = VariantType {
        ord: 29i32
    };
    #[doc(alias = "TYPE_PACKED_INT32_ARRAY")]
    #[doc = "Godot enumerator name: `TYPE_PACKED_INT32_ARRAY`"]
    pub const PACKED_INT32_ARRAY: VariantType = VariantType {
        ord: 30i32
    };
    #[doc(alias = "TYPE_PACKED_INT64_ARRAY")]
    #[doc = "Godot enumerator name: `TYPE_PACKED_INT64_ARRAY`"]
    pub const PACKED_INT64_ARRAY: VariantType = VariantType {
        ord: 31i32
    };
    #[doc(alias = "TYPE_PACKED_FLOAT32_ARRAY")]
    #[doc = "Godot enumerator name: `TYPE_PACKED_FLOAT32_ARRAY`"]
    pub const PACKED_FLOAT32_ARRAY: VariantType = VariantType {
        ord: 32i32
    };
    #[doc(alias = "TYPE_PACKED_FLOAT64_ARRAY")]
    #[doc = "Godot enumerator name: `TYPE_PACKED_FLOAT64_ARRAY`"]
    pub const PACKED_FLOAT64_ARRAY: VariantType = VariantType {
        ord: 33i32
    };
    #[doc(alias = "TYPE_PACKED_STRING_ARRAY")]
    #[doc = "Godot enumerator name: `TYPE_PACKED_STRING_ARRAY`"]
    pub const PACKED_STRING_ARRAY: VariantType = VariantType {
        ord: 34i32
    };
    #[doc(alias = "TYPE_PACKED_VECTOR2_ARRAY")]
    #[doc = "Godot enumerator name: `TYPE_PACKED_VECTOR2_ARRAY`"]
    pub const PACKED_VECTOR2_ARRAY: VariantType = VariantType {
        ord: 35i32
    };
    #[doc(alias = "TYPE_PACKED_VECTOR3_ARRAY")]
    #[doc = "Godot enumerator name: `TYPE_PACKED_VECTOR3_ARRAY`"]
    pub const PACKED_VECTOR3_ARRAY: VariantType = VariantType {
        ord: 36i32
    };
    #[doc(alias = "TYPE_PACKED_COLOR_ARRAY")]
    #[doc = "Godot enumerator name: `TYPE_PACKED_COLOR_ARRAY`"]
    pub const PACKED_COLOR_ARRAY: VariantType = VariantType {
        ord: 37i32
    };
    #[doc(alias = "TYPE_PACKED_VECTOR4_ARRAY")]
    #[doc = "Godot enumerator name: `TYPE_PACKED_VECTOR4_ARRAY`"]
    pub const PACKED_VECTOR4_ARRAY: VariantType = VariantType {
        ord: 38i32
    };
    #[doc(alias = "TYPE_MAX")]
    #[doc = "Godot enumerator name: `TYPE_MAX`"]
    pub const MAX: VariantType = VariantType {
        ord: 39i32
    };
    
}
impl std::fmt::Debug for VariantType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::NIL => "NIL", Self::BOOL => "BOOL", Self::INT => "INT", Self::FLOAT => "FLOAT", Self::STRING => "STRING", Self::VECTOR2 => "VECTOR2", Self::VECTOR2I => "VECTOR2I", Self::RECT2 => "RECT2", Self::RECT2I => "RECT2I", Self::VECTOR3 => "VECTOR3", Self::VECTOR3I => "VECTOR3I", Self::TRANSFORM2D => "TRANSFORM2D", Self::VECTOR4 => "VECTOR4", Self::VECTOR4I => "VECTOR4I", Self::PLANE => "PLANE", Self::QUATERNION => "QUATERNION", Self::AABB => "AABB", Self::BASIS => "BASIS", Self::TRANSFORM3D => "TRANSFORM3D", Self::PROJECTION => "PROJECTION", Self::COLOR => "COLOR", Self::STRING_NAME => "STRING_NAME", Self::NODE_PATH => "NODE_PATH", Self::RID => "RID", Self::OBJECT => "OBJECT", Self::CALLABLE => "CALLABLE", Self::SIGNAL => "SIGNAL", Self::DICTIONARY => "DICTIONARY", Self::ARRAY => "ARRAY", Self::PACKED_BYTE_ARRAY => "PACKED_BYTE_ARRAY", Self::PACKED_INT32_ARRAY => "PACKED_INT32_ARRAY", Self::PACKED_INT64_ARRAY => "PACKED_INT64_ARRAY", Self::PACKED_FLOAT32_ARRAY => "PACKED_FLOAT32_ARRAY", Self::PACKED_FLOAT64_ARRAY => "PACKED_FLOAT64_ARRAY", Self::PACKED_STRING_ARRAY => "PACKED_STRING_ARRAY", Self::PACKED_VECTOR2_ARRAY => "PACKED_VECTOR2_ARRAY", Self::PACKED_VECTOR3_ARRAY => "PACKED_VECTOR3_ARRAY", Self::PACKED_COLOR_ARRAY => "PACKED_COLOR_ARRAY", Self::PACKED_VECTOR4_ARRAY => "PACKED_VECTOR4_ARRAY", Self::MAX => "MAX", _ => {
                f.debug_struct("VariantType") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl VariantType {
    #[doc(hidden)]
    pub fn from_sys(enumerator: crate::GDExtensionVariantType) -> Self {
        #[allow(clippy::unnecessary_cast)]
        Self {
            ord: enumerator as i32
        }
    }
    #[doc(hidden)]
    pub fn sys(self) -> crate::GDExtensionVariantType {
        self.ord as _
    }
}