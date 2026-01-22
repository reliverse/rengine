// Source: https://gtamods.com/wiki/RenderWare

export default class RwVersion {
  static readonly versions: { [versionNumber: number]: string } = {
    200704: "RenderWare 3.1.0.0 (III on PS2)",
    204800: "RenderWare 3.2.0.0 (III on PC)",
    208898: "RenderWare 3.3.0.2 (III on PC, VC on PS2)",
    212995: "RenderWare 3.4.0.3 (VC on PC)",
    212997: "RenderWare 3.4.0.5 (III on PS2, VC on Android/PC)",
    217088: "RenderWare 3.5.0.0 (III/VC on Xbox)",
    221187: "RenderWare 3.6.0.3 (SA)",
  };

  public static unpackVersion(version: number) {
    if (version & 0xff_ff_00_00) {
      return (((version >> 14) & 0x3_ff_00) + 0x3_00_00) | ((version >> 16) & 0x3f);
    }
    return version;
  }

  public static unpackBuild(version: number) {
    if (version & 0xff_ff_00_00) {
      return version & 0xff_ff;
    }
    return 0;
  }
}
