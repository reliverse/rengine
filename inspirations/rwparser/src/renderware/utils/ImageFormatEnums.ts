export enum RasterFormat {
  RASTER_1555 = 0x01_00,
  RASTER_565 = 0x02_00,
  RASTER_4444 = 0x03_00,
  RASTER_LUM = 0x04_00,
  RASTER_8888 = 0x05_00,
  RASTER_888 = 0x06_00,
  RASTER_555 = 0x0a_00,
}

export enum D3DFormat {
  D3DFMT_A8L8 = "3",
  D3D_DXT1 = "DXT1",
  D3D_DXT2 = "DXT2",
  D3D_DXT3 = "DXT3",
  D3D_DXT4 = "DXT4",
  D3D_DXT5 = "DXT5",
}

export enum PaletteType {
  PALETTE_NONE = 0,
  PALETTE_8 = 1,
}

export enum PlatformType {
  D3D8 = 0x8,
  D3D9 = 0x9,
}
