export enum RwSections {
  // Core
  RwStruct = 0x00_01,
  RwString = 0x00_02,
  RwExtension = 0x00_03,
  RwTexture = 0x00_06,
  RwMaterial = 0x00_07,
  RwMaterialList = 0x00_08,
  RwFrameList = 0x00_0e,
  RwGeometry = 0x00_0f,
  RwClump = 0x00_10,
  RwAtomic = 0x00_14,
  RwTextureNative = 0x00_15,
  RwTextureDictionary = 0x00_16,
  RwGeometryList = 0x00_1a,
  RwBinMesh = 0x5_0e,
  RwSkin = 0x1_16,
  RwAnim = 0x1_1e,

  // Toolkit
  RwMaterialEffectsPLG = 0x01_20,

  // R* specific RW plugins
  RwReflectionMaterial = 0x02_53_f2_fc,
  // This was renamed to RwNodeName from RwFrame to prevent confusion.
  // https://gtamods.com/wiki/Node_Name_(RW_Section)
  RwNodeName = 0x02_53_f2_fe,
}
