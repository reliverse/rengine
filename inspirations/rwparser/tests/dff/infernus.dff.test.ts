import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { DffParser, type RwDff } from "../../src/index";

describe("dff parsing - infernus", () => {
  const FLOATING_POINT_ERROR = 6;

  let rwDff: RwDff;

  beforeAll(async () => {
    // Disable console debug for now, because of missing structures
    const consoleDebug = console.debug;
    console.debug = jest.fn();

    const dffParser = new DffParser(
      await readFile(join(__dirname, "../assets/infernus.dff"))
    );
    rwDff = dffParser.parse();

    console.debug = consoleDebug;
  });

  test("SA version", () => {
    expect(rwDff.version).toBe("RenderWare 3.6.0.3 (SA)");
    expect(rwDff.versionNumber).toBe(0x3_60_03);
  });

  test("atomics - length", () => {
    expect(rwDff.clumps[0].atomics).toHaveLength(15);
  });

  test("atomics - index matching", () => {
    if (!rwDff.clumps[0].atomics || rwDff.clumps[0].atomics.length === 0) {
      throw new Error("Atomics is missing");
    }
    expect(rwDff.clumps[0].atomics[1].frameIndex).toBe(14);
    expect(rwDff.clumps[0].atomics[4].frameIndex).toBe(31);
    expect(rwDff.clumps[0].atomics[14].frameIndex).toBe(21);
  });

  test("dummies - length", () => {
    expect(rwDff.clumps[0].dummies).toHaveLength(36);
  });

  test("dummies - names", () => {
    expect(rwDff.clumps[0].dummies[0]).toBe("infernus");
    expect(rwDff.clumps[0].dummies[2]).toBe("wheel_lb_dummy");
    expect(rwDff.clumps[0].dummies[3]).toBe("wheel_rf_dummy");
    expect(rwDff.clumps[0].dummies[35]).toBe("wheel");
  });

  test("frames - length", () => {
    const frameList = rwDff.clumps[0].frameList;

    expect(frameList).toBeDefined();
    expect(frameList?.frameCount).toBe(36);
    expect(frameList?.frames).toHaveLength(frameList?.frameCount);
  });

  test("frames - parent frames", () => {
    const frameList = rwDff.clumps[0].frameList;

    expect(frameList?.frames[0].parentFrame).toBe(-1);
    expect(frameList?.frames[1].parentFrame).toBe(0);
    expect(frameList?.frames[2].parentFrame).toBe(0);
    expect(frameList?.frames[3].parentFrame).toBe(0);
    expect(frameList?.frames[4].parentFrame).toBe(0);
    expect(frameList?.frames[35].parentFrame).toBe(3);
  });

  test("frames - coordinate offsets", () => {
    const frameList = rwDff.clumps[0].frameList;

    expect(frameList?.frames[0].coordinatesOffset).toStrictEqual({
      x: 0,
      y: 0,
      z: 0,
    });
    expect(frameList?.frames[1].coordinatesOffset).toStrictEqual({
      x: expect.closeTo(0.704_930_067_062_377_9, FLOATING_POINT_ERROR),
      y: expect.closeTo(2.682_444_810_867_309_6, FLOATING_POINT_ERROR),
      z: expect.closeTo(-0.190_898_478_031_158_45, FLOATING_POINT_ERROR),
    });
    expect(frameList?.frames[35].coordinatesOffset).toStrictEqual({
      x: 0,
      y: 0,
      z: 0,
    });
  });

  test("frames - rotation matrices", () => {
    const frameList = rwDff.clumps[0].frameList;

    expect(frameList?.frames[0].rotationMatrix).toStrictEqual({
      right: { x: 1, y: 0, z: 0 },
      up: { x: 0, y: 1, z: 0 },
      at: { x: 0, y: 0, z: 1 },
    });
    expect(frameList?.frames[20].rotationMatrix).toStrictEqual({
      right: { x: 1, y: 0, z: 0 },
      up: { x: 0, y: 1, z: 0 },
      at: { x: 0, y: 0, z: 1 },
    });
    expect(frameList?.frames[35].rotationMatrix).toStrictEqual({
      right: { x: 1, y: 0, z: 0 },
      up: { x: 0, y: 1, z: 0 },
      at: { x: 0, y: 0, z: 1 },
    });
  });

  test("geometries - length", () => {
    const geometryList = rwDff.clumps[0].geometryList;

    expect(geometryList).toBeDefined();
    expect(geometryList?.geometricObjectCount).toBe(15);
    expect(geometryList?.geometries).toHaveLength(
      geometryList?.geometricObjectCount
    );
  });

  test("geometries - vertices", () => {
    const geometryList = rwDff.clumps[0].geometryList;

    expect(geometryList?.geometries[0].hasVertices).toBeTruthy();
    expect(geometryList?.geometries[14].hasVertices).toBeTruthy();

    expect(geometryList?.geometries[0].vertexInformation).toHaveLength(298);
    expect(geometryList?.geometries[14].vertexInformation).toHaveLength(1970);

    expect(geometryList?.geometries[0].vertexInformation[0]).toStrictEqual({
      x: expect.closeTo(0.011_610_686_779_022_217, FLOATING_POINT_ERROR),
      y: expect.closeTo(-0.340_093_821_287_155_15, FLOATING_POINT_ERROR),
      z: expect.closeTo(6.604_368_252_283_166e-8, FLOATING_POINT_ERROR),
    });
    expect(geometryList?.geometries[0].vertexInformation[4]).toStrictEqual({
      x: expect.closeTo(0.144_699_573_516_845_7, FLOATING_POINT_ERROR),
      y: expect.closeTo(-0.258_058_756_589_889_5, FLOATING_POINT_ERROR),
      z: expect.closeTo(8.390_583_872_142_088e-8, FLOATING_POINT_ERROR),
    });
    expect(geometryList?.geometries[0].vertexInformation[297]).toStrictEqual({
      x: expect.closeTo(-0.145_337_358_117_103_58, FLOATING_POINT_ERROR),
      y: expect.closeTo(-0.088_308_162_987_232_21, FLOATING_POINT_ERROR),
      z: expect.closeTo(0.036_604_382_097_721_1, FLOATING_POINT_ERROR),
    });

    expect(geometryList?.geometries[14].vertexInformation[0]).toStrictEqual({
      x: expect.closeTo(0.307_336_449_623_107_9, FLOATING_POINT_ERROR),
      y: expect.closeTo(-2.509_900_569_915_771_5, FLOATING_POINT_ERROR),
      z: expect.closeTo(-0.155_147_492_885_589_6, FLOATING_POINT_ERROR),
    });
    expect(geometryList?.geometries[14].vertexInformation[4]).toStrictEqual({
      x: expect.closeTo(0.165_699_899_196_624_76, FLOATING_POINT_ERROR),
      y: expect.closeTo(0.499_986_439_943_313_6, FLOATING_POINT_ERROR),
      z: expect.closeTo(0.608_757_615_089_416_5, FLOATING_POINT_ERROR),
    });
    expect(geometryList?.geometries[14].vertexInformation[1969]).toStrictEqual({
      x: expect.closeTo(0.997_877_597_808_837_9, FLOATING_POINT_ERROR),
      y: expect.closeTo(-2.445_856_332_778_930_7, FLOATING_POINT_ERROR),
      z: expect.closeTo(0.184_027_343_988_418_58, FLOATING_POINT_ERROR),
    });
  });

  test("geometries - normals", () => {
    const geometryList = rwDff.clumps[0].geometryList;

    expect(geometryList?.geometries[0].hasNormals).toBeTruthy();
    expect(geometryList?.geometries[14].hasNormals).toBeTruthy();

    expect(geometryList?.geometries[0].normalInformation).toHaveLength(298);
    expect(geometryList?.geometries[14].normalInformation).toHaveLength(1970);

    expect(geometryList?.geometries[0].normalInformation[0]).toStrictEqual({
      x: 1,
      y: expect.closeTo(-4.381_484_686_177_828e-8, FLOATING_POINT_ERROR),
      z: expect.closeTo(1.095_317_616_445_733_2e-14, FLOATING_POINT_ERROR),
    });
    expect(geometryList?.geometries[0].normalInformation[4]).toStrictEqual({
      x: expect.closeTo(0.985_611_379_146_575_9, FLOATING_POINT_ERROR),
      y: expect.closeTo(0.169_027_328_491_210_94, FLOATING_POINT_ERROR),
      z: expect.closeTo(-6.338_092_362_057_068e-7, FLOATING_POINT_ERROR),
    });
    expect(geometryList?.geometries[0].normalInformation[297]).toStrictEqual({
      x: expect.closeTo(-0.668_809_652_328_491_2, FLOATING_POINT_ERROR),
      y: expect.closeTo(-0.282_025_724_649_429_3, FLOATING_POINT_ERROR),
      z: expect.closeTo(0.687_862_753_868_103, FLOATING_POINT_ERROR),
    });

    expect(geometryList?.geometries[14].normalInformation[0]).toStrictEqual({
      x: expect.closeTo(0.007_420_234_382_152_557, FLOATING_POINT_ERROR),
      y: expect.closeTo(-0.727_088_987_827_301, FLOATING_POINT_ERROR),
      z: expect.closeTo(0.686_503_171_920_776_4, FLOATING_POINT_ERROR),
    });
    expect(geometryList?.geometries[14].normalInformation[4]).toStrictEqual({
      x: expect.closeTo(0.027_043_940_499_424_934, FLOATING_POINT_ERROR),
      y: expect.closeTo(0.328_894_674_777_984_6, FLOATING_POINT_ERROR),
      z: expect.closeTo(0.943_979_322_910_308_8, FLOATING_POINT_ERROR),
    });
    expect(geometryList?.geometries[14].normalInformation[1969]).toStrictEqual({
      x: expect.closeTo(0.123_670_719_563_961_03, FLOATING_POINT_ERROR),
      y: expect.closeTo(-0.990_243_732_929_229_7, FLOATING_POINT_ERROR),
      z: expect.closeTo(0.064_210_087_060_928_34, FLOATING_POINT_ERROR),
    });
  });

  test("geometries - triangles", () => {
    const geometryList = rwDff.clumps[0].geometryList;

    expect(geometryList?.geometries[0].triangleInformation).toHaveLength(250);
    expect(geometryList?.geometries[14].triangleInformation).toHaveLength(1710);

    expect(geometryList?.geometries[0].triangleInformation[0]).toStrictEqual({
      vector: { x: 0, y: 3, z: 2 },
      materialId: 0,
    });
    expect(geometryList?.geometries[0].triangleInformation[249]).toStrictEqual({
      vector: { x: 239, y: 245, z: 243 },
      materialId: 3,
    });

    expect(geometryList?.geometries[14].triangleInformation[0]).toStrictEqual({
      vector: { x: 789, y: 787, z: 345 },
      materialId: 0,
    });
    expect(
      geometryList?.geometries[14].triangleInformation[1709]
    ).toStrictEqual({
      vector: { x: 1923, y: 1924, z: 1921 },
      materialId: 15,
    });
  });

  test("geometries - vertex colors", () => {
    const geometryList = rwDff.clumps[0].geometryList;

    expect(geometryList?.geometries[0].vertexColorInformation).toHaveLength(0);
    expect(geometryList?.geometries[14].vertexColorInformation).toHaveLength(0);
  });

  test("geometries - vertex colors", () => {
    const geometryList = rwDff.clumps[0].geometryList;

    expect(geometryList?.geometries[0].vertexColorInformation).toHaveLength(0);
    expect(geometryList?.geometries[14].vertexColorInformation).toHaveLength(0);
  });

  test("geometries - texture mapping", () => {
    const geometryList = rwDff.clumps[0].geometryList;

    expect(geometryList?.geometries[0].textureCoordinatesCount).toBe(1);
    expect(geometryList?.geometries[3].textureCoordinatesCount).toBe(1);
    expect(geometryList?.geometries[14].textureCoordinatesCount).toBe(2);

    expect(geometryList?.geometries[0].textureMappingInformation).toHaveLength(
      1
    );
    expect(geometryList?.geometries[3].textureMappingInformation).toHaveLength(
      1
    );
    expect(geometryList?.geometries[14].textureMappingInformation).toHaveLength(
      2
    );

    expect(
      geometryList?.geometries[0].textureMappingInformation[0]
    ).toHaveLength(298);

    expect(
      geometryList?.geometries[0].textureMappingInformation[0][0]
    ).toStrictEqual({ u: 0, v: 0 });
    expect(
      geometryList?.geometries[0].textureMappingInformation[0][3]
    ).toStrictEqual({ u: 0, v: 0 });
    expect(
      geometryList?.geometries[0].textureMappingInformation[0][4]
    ).toStrictEqual({
      u: expect.closeTo(0.251_775_175_333_023_07, FLOATING_POINT_ERROR),
      v: expect.closeTo(0.219_771_802_425_384_52, FLOATING_POINT_ERROR),
    });
    expect(
      geometryList?.geometries[0].textureMappingInformation[0][297]
    ).toStrictEqual({
      u: expect.closeTo(0.472_493_141_889_572_14, FLOATING_POINT_ERROR),
      v: expect.closeTo(0.731_933_474_540_710_4, FLOATING_POINT_ERROR),
    });

    expect(
      geometryList?.geometries[14].textureMappingInformation[0]
    ).toHaveLength(1970);
    expect(
      geometryList?.geometries[14].textureMappingInformation[1]
    ).toHaveLength(1970);

    expect(
      geometryList?.geometries[14].textureMappingInformation[0][0]
    ).toStrictEqual({
      u: expect.closeTo(0.566_294_968_128_204_3, FLOATING_POINT_ERROR),
      v: expect.closeTo(0.304_455_459_117_889_4, FLOATING_POINT_ERROR),
    });
    expect(
      geometryList?.geometries[14].textureMappingInformation[0][1969]
    ).toStrictEqual({
      u: expect.closeTo(0.599_575_400_352_478, FLOATING_POINT_ERROR),
      v: expect.closeTo(0.922_805_845_737_457_3, FLOATING_POINT_ERROR),
    });

    expect(
      geometryList?.geometries[14].textureMappingInformation[1][0]
    ).toStrictEqual({
      u: expect.closeTo(0.983_111_619_949_340_8, FLOATING_POINT_ERROR),
      v: expect.closeTo(2.838_337_659_835_815_4, FLOATING_POINT_ERROR),
    });
    expect(
      geometryList?.geometries[14].textureMappingInformation[1][1969]
    ).toStrictEqual({
      u: expect.closeTo(0.971_399_664_878_845_2, FLOATING_POINT_ERROR),
      v: expect.closeTo(2.716_962_337_493_896_5, FLOATING_POINT_ERROR),
    });
  });

  test("geometries - materials", () => {
    const geometryList = rwDff.clumps[0].geometryList;

    expect(geometryList?.geometries[0].materialList.materialInstanceCount).toBe(
      4
    );
    expect(
      geometryList?.geometries[14].materialList.materialInstanceCount
    ).toBe(16);

    expect(geometryList?.geometries[0].materialList.materialData).toHaveLength(
      4
    );
    expect(geometryList?.geometries[14].materialList.materialData).toHaveLength(
      16
    );

    expect(
      geometryList?.geometries[0].materialList.materialData[0].isTextured
    ).toBeFalsy();
    expect(
      geometryList?.geometries[0].materialList.materialData[0].color
    ).toStrictEqual({ r: 0, g: 0, b: 0, a: 255 });
    expect(
      geometryList?.geometries[0].materialList.materialData[0].ambient
    ).toBe(1);
    expect(
      geometryList?.geometries[0].materialList.materialData[0].diffuse
    ).toBe(1);
    expect(
      geometryList?.geometries[0].materialList.materialData[0].specular
    ).toBe(0);
    expect(
      geometryList?.geometries[0].materialList.materialData[0].texture
    ).toBeUndefined();

    expect(
      geometryList?.geometries[0].materialList.materialData[1].isTextured
    ).toBeTruthy();
    expect(
      geometryList?.geometries[0].materialList.materialData[1].color
    ).toStrictEqual({ r: 255, g: 255, b: 255, a: 255 });
    expect(
      geometryList?.geometries[0].materialList.materialData[1].ambient
    ).toBe(0.5);

    expect(
      geometryList?.geometries[0].materialList.materialData[2].texture
    ).toBeDefined();
    expect(
      geometryList?.geometries[0].materialList.materialData[2].texture
    ).toStrictEqual({
      textureFiltering: 6,
      textureName: "infernus92wheel32",
      uAddressing: 1,
      vAddressing: 1,
      usesMipLevels: true,
    });

    expect(
      geometryList?.geometries[0].materialList.materialData[3].isTextured
    ).toBeTruthy();
    expect(
      geometryList?.geometries[0].materialList.materialData[3].ambient
    ).toBeCloseTo(0.400_000_005_960_464_5, FLOATING_POINT_ERROR);
    expect(
      geometryList?.geometries[0].materialList.materialData[3].texture
    ).toBeDefined();
    expect(
      geometryList?.geometries[0].materialList.materialData[3].texture
    ).toStrictEqual({
      textureFiltering: 6,
      textureName: "infernus92interior128",
      uAddressing: 1,
      vAddressing: 1,
      usesMipLevels: true,
    });
  });

  test("geometries - bin mesh", () => {
    const geometryList = rwDff.clumps[0].geometryList;

    expect(geometryList?.geometries[0].binMesh?.meshCount).toBe(4);
    expect(geometryList?.geometries[14].binMesh?.meshCount).toBe(16);

    expect(geometryList?.geometries[0].binMesh?.meshes).toHaveLength(4);
    expect(geometryList?.geometries[14].binMesh?.meshes).toHaveLength(16);

    expect(geometryList?.geometries[0].binMesh?.meshes[0].indexCount).toBe(4);
    expect(geometryList?.geometries[0].binMesh?.meshes[3].indexCount).toBe(210);

    expect(geometryList?.geometries[0].binMesh?.meshes[0].indices).toHaveLength(
      4
    );
    expect(geometryList?.geometries[0].binMesh?.meshes[3].indices).toHaveLength(
      210
    );

    expect(geometryList?.geometries[14].binMesh?.meshes[15].indices[0]).toBe(
      1937
    );
    expect(geometryList?.geometries[14].binMesh?.meshes[15].indices[100]).toBe(
      1909
    );

    expect(geometryList?.geometries[14].binMesh?.meshes[14].materialIndex).toBe(
      11
    );
  });

  test("geometries - bounding sphere", () => {
    const geometryList = rwDff.clumps[0].geometryList;

    expect(geometryList?.geometries[0].boundingSphere).toStrictEqual({
      vector: {
        x: expect.closeTo(0.000_050_142_407_417_297_36, FLOATING_POINT_ERROR),
        y: expect.closeTo(1.490_116_119_384_765_6e-8, FLOATING_POINT_ERROR),
        z: expect.closeTo(1.490_116_119_384_765_6e-8, FLOATING_POINT_ERROR),
      },
      radius: expect.closeTo(0.371_361_941_099_166_87, FLOATING_POINT_ERROR),
    });
    expect(geometryList?.geometries[14].boundingSphere).toStrictEqual({
      vector: {
        x: expect.closeTo(-5.364_418_029_785_156e-7, FLOATING_POINT_ERROR),
        y: expect.closeTo(0.158_847_689_628_601_07, FLOATING_POINT_ERROR),
        z: expect.closeTo(0.011_729_836_463_928_223, FLOATING_POINT_ERROR),
      },
      radius: expect.closeTo(2.849_857_807_159_424, FLOATING_POINT_ERROR),
    });
  });
});
