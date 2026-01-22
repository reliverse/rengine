import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { DffParser, type RwDff } from "../../src/index";

// This test for skin and bones sections
describe("dff parsing - wuzimu", () => {
  const FLOATING_POINT_ERROR = 6;
  let rwDff: RwDff;

  beforeAll(async () => {
    const consoleDebug = console.debug;
    console.debug = jest.fn();

    const dffParser = new DffParser(
      await readFile(join(__dirname, "../assets/wuzimu.dff"))
    );
    rwDff = dffParser.parse();

    console.debug = consoleDebug;
  });

  test("SA version", () => {
    expect(rwDff.version).toBe("RenderWare 3.6.0.3 (SA)");
    expect(rwDff.versionNumber).toBe(0x3_60_03);
  });

  test("dummies - length", () => {
    expect(rwDff.clumps[0].dummies).toHaveLength(32);
  });

  test("dummies - names", () => {
    expect(rwDff.clumps[0].dummies[0]).toBe("Normal");
    expect(rwDff.clumps[0].dummies[1]).toBe(" Pelvis");
    expect(rwDff.clumps[0].dummies[2]).toBe(" R Thigh");
    expect(rwDff.clumps[0].dummies[15]).toBe("Jaw");
    expect(rwDff.clumps[0].dummies[30]).toBe(" R Foot");
    expect(rwDff.clumps[0].dummies[31]).toBe(" R Toe0");
  });

  test("frames - length", () => {
    const frameList = rwDff.clumps[0].frameList;

    expect(frameList).toBeDefined();
    expect(frameList?.frameCount).toBe(33);
    expect(frameList?.frames).toHaveLength(frameList?.frameCount);
  });

  test("frames - parent frames", () => {
    const frameList = rwDff.clumps[0].frameList;

    expect(frameList?.frames[0].parentFrame).toBe(-1);
    expect(frameList?.frames[1].parentFrame).toBe(0);
    expect(frameList?.frames[2].parentFrame).toBe(1);
    expect(frameList?.frames[3].parentFrame).toBe(2);
    expect(frameList?.frames[6].parentFrame).toBe(5);
    expect(frameList?.frames[15].parentFrame).toBe(13);
    expect(frameList?.frames[31].parentFrame).toBe(30);
    expect(frameList?.frames[32].parentFrame).toBe(31);
  });

  test("frames - coordinate offsets", () => {
    const frameList = rwDff.clumps[0].frameList;

    expect(frameList?.frames[0].coordinatesOffset).toStrictEqual({
      x: 0,
      y: 0,
      z: 0,
    });
    expect(frameList?.frames[1].coordinatesOffset).toStrictEqual({
      x: 0,
      y: 0,
      z: 0,
    });
    expect(frameList?.frames[16].coordinatesOffset).toStrictEqual({
      x: expect.closeTo(0.011_230_167_001_485_825, FLOATING_POINT_ERROR),
      y: expect.closeTo(0.015_464_058_145_880_7, FLOATING_POINT_ERROR),
      z: expect.closeTo(-0.004_240_759_648_382_664, FLOATING_POINT_ERROR),
    });
    expect(frameList?.frames[32].coordinatesOffset).toStrictEqual({
      x: expect.closeTo(0.102_001_182_734_966_28, FLOATING_POINT_ERROR),
      y: expect.closeTo(0.154_107_719_659_805_3, FLOATING_POINT_ERROR),
      z: expect.closeTo(0.0, FLOATING_POINT_ERROR),
    });
  });

  test("frames - rotation matrices", () => {
    const frameList = rwDff.clumps[0].frameList;

    expect(frameList?.frames[0].rotationMatrix).toStrictEqual({
      right: { x: 1, y: 0, z: 0 },
      up: { x: 0, y: 1, z: 0 },
      at: { x: 0, y: 0, z: 1 },
    });
    expect(frameList?.frames[10].rotationMatrix).toStrictEqual({
      right: {
        x: expect.closeTo(0.978_540_778_160_095_2, FLOATING_POINT_ERROR),
        y: expect.closeTo(0.206_053_182_482_719_42, FLOATING_POINT_ERROR),
        z: expect.closeTo(0.0, FLOATING_POINT_ERROR),
      },
      up: {
        x: expect.closeTo(-0.206_053_152_680_397_03, FLOATING_POINT_ERROR),
        y: expect.closeTo(0.978_540_837_764_74, FLOATING_POINT_ERROR),
        z: expect.closeTo(0.0, FLOATING_POINT_ERROR),
      },
      at: {
        x: expect.closeTo(0.0, FLOATING_POINT_ERROR),
        y: expect.closeTo(0.0, FLOATING_POINT_ERROR),
        z: expect.closeTo(1.0, FLOATING_POINT_ERROR),
      },
    });
    expect(frameList?.frames[32].rotationMatrix).toStrictEqual({
      right: { x: 0, y: 1, z: expect.closeTo(0.0, FLOATING_POINT_ERROR) },
      up: { x: -1, y: 0, z: 0 },
      at: { x: 0, y: 0, z: 1 },
    });
  });

  test("bones - length", () => {
    expect(rwDff.clumps[0].animNodes.length).toStrictEqual(32);
    expect(rwDff.clumps[0].animNodes[0].bones.length).toStrictEqual(32);
  });

  test("bones - ids", () => {
    expect(rwDff.clumps[0].animNodes[0].boneId).toStrictEqual(0);
    expect(rwDff.clumps[0].animNodes[1].boneId).toStrictEqual(1);
    expect(rwDff.clumps[0].animNodes[2].boneId).toStrictEqual(51);
    expect(rwDff.clumps[0].animNodes[3].boneId).toStrictEqual(41);
    expect(rwDff.clumps[0].animNodes[15].boneId).toStrictEqual(8);
    expect(rwDff.clumps[0].animNodes[30].boneId).toStrictEqual(53);
    expect(rwDff.clumps[0].animNodes[31].boneId).toStrictEqual(54);
    expect(rwDff.clumps[0].animNodes[0].bones[0].boneId).toStrictEqual(0);
    expect(rwDff.clumps[0].animNodes[0].bones[2].boneId).toStrictEqual(2);
    expect(rwDff.clumps[0].animNodes[0].bones[7].boneId).toStrictEqual(6);
    expect(rwDff.clumps[0].animNodes[0].bones[18].boneId).toStrictEqual(24);
    expect(rwDff.clumps[0].animNodes[0].bones[31].boneId).toStrictEqual(54);
  });

  test("bones - bone count", () => {
    expect(rwDff.clumps[0].animNodes[0].bonesCount).toStrictEqual(32);
    expect(rwDff.clumps[0].animNodes[3].bonesCount).toStrictEqual(0);
    expect(rwDff.clumps[0].animNodes[15].bonesCount).toStrictEqual(0);
    expect(rwDff.clumps[0].animNodes[31].bonesCount).toStrictEqual(0);
  });

  test("skin - bone count", () => {
    const skin = rwDff.clumps[0].geometryList?.geometries[0].skin!;

    expect(skin.boneCount).toStrictEqual(32);
    expect(skin.usedBoneCount).toStrictEqual(31);
  });

  test("skin - vertex weights", () => {
    const skin = rwDff.clumps[0].geometryList?.geometries[0].skin!;

    expect(skin.maxWeightsPerVertex).toStrictEqual(4);
    expect(skin.vertexWeights.length).toStrictEqual(990);

    expect(skin.vertexWeights[0][0]).toBeCloseTo(
      0.577_356_338_500_976_6,
      FLOATING_POINT_ERROR
    );
    expect(skin.vertexWeights[0][1]).toBeCloseTo(
      0.422_643_661_499_023_44,
      FLOATING_POINT_ERROR
    );
    expect(skin.vertexWeights[0][2]).toBeCloseTo(0, FLOATING_POINT_ERROR);
    expect(skin.vertexWeights[0][3]).toBeCloseTo(0, FLOATING_POINT_ERROR);

    expect(skin.vertexWeights[654][0]).toBeCloseTo(1, FLOATING_POINT_ERROR);
    expect(skin.vertexWeights[654][1]).toBeCloseTo(0, FLOATING_POINT_ERROR);
    expect(skin.vertexWeights[654][2]).toBeCloseTo(0, FLOATING_POINT_ERROR);
    expect(skin.vertexWeights[654][3]).toBeCloseTo(0, FLOATING_POINT_ERROR);

    expect(skin.vertexWeights[989][0]).toBeCloseTo(1, FLOATING_POINT_ERROR);
    expect(skin.vertexWeights[989][1]).toBeCloseTo(0, FLOATING_POINT_ERROR);
    expect(skin.vertexWeights[989][2]).toBeCloseTo(0, FLOATING_POINT_ERROR);
    expect(skin.vertexWeights[989][3]).toBeCloseTo(0, FLOATING_POINT_ERROR);
  });

  test("skin - bone-vertex map", () => {
    const skin = rwDff.clumps[0].geometryList?.geometries[0].skin!;

    expect(skin.boneVertexIndices[0][0]).toStrictEqual(28);
    expect(skin.boneVertexIndices[0][1]).toStrictEqual(24);
    expect(skin.boneVertexIndices[0][2]).toStrictEqual(0);
    expect(skin.boneVertexIndices[0][3]).toStrictEqual(0);

    expect(skin.boneVertexIndices[525][0]).toStrictEqual(5);
    expect(skin.boneVertexIndices[525][1]).toStrictEqual(6);
    expect(skin.boneVertexIndices[525][2]).toStrictEqual(0);
    expect(skin.boneVertexIndices[525][3]).toStrictEqual(0);

    expect(skin.boneVertexIndices[989][0]).toStrictEqual(5);
    expect(skin.boneVertexIndices[989][1]).toStrictEqual(0);
    expect(skin.boneVertexIndices[989][2]).toStrictEqual(0);
    expect(skin.boneVertexIndices[989][3]).toStrictEqual(0);
  });

  test("skin - inverse bone matrices", () => {
    const skin = rwDff.clumps[0].geometryList?.geometries[0].skin!;

    expect(skin.inverseBoneMatrices.length).toStrictEqual(32);

    expect(skin.inverseBoneMatrices[0]).toStrictEqual({
      right: {
        x: expect.closeTo(1, FLOATING_POINT_ERROR),
        y: expect.closeTo(0, FLOATING_POINT_ERROR),
        z: expect.closeTo(0, FLOATING_POINT_ERROR),
        t: expect.closeTo(0, FLOATING_POINT_ERROR),
      },
      up: {
        x: expect.closeTo(0, FLOATING_POINT_ERROR),
        y: expect.closeTo(1, FLOATING_POINT_ERROR),
        z: expect.closeTo(0, FLOATING_POINT_ERROR),
        t: expect.closeTo(0, FLOATING_POINT_ERROR),
      },
      at: {
        x: expect.closeTo(0, FLOATING_POINT_ERROR),
        y: expect.closeTo(0, FLOATING_POINT_ERROR),
        z: expect.closeTo(1, FLOATING_POINT_ERROR),
        t: expect.closeTo(1.817_587_224_417_495_8e-20, FLOATING_POINT_ERROR),
      },
      transform: {
        x: expect.closeTo(0, FLOATING_POINT_ERROR),
        y: expect.closeTo(0, FLOATING_POINT_ERROR),
        z: expect.closeTo(0, FLOATING_POINT_ERROR),
        t: expect.closeTo(1.485_376_372_184_306e-43, FLOATING_POINT_ERROR),
      },
    });

    expect(skin.inverseBoneMatrices[31]).toStrictEqual({
      right: {
        x: expect.closeTo(1, FLOATING_POINT_ERROR),
        y: expect.closeTo(-9.176_544_324_418_501e-8, FLOATING_POINT_ERROR),
        z: expect.closeTo(-9.583_585_314_221_21e-11, FLOATING_POINT_ERROR),
        t: expect.closeTo(0, FLOATING_POINT_ERROR),
      },
      up: {
        x: expect.closeTo(-9.313_955_162_681_964e-10, FLOATING_POINT_ERROR),
        y: expect.closeTo(-1.222_420_031_865_567_6e-8, FLOATING_POINT_ERROR),
        z: expect.closeTo(-1, FLOATING_POINT_ERROR),
        t: expect.closeTo(0, FLOATING_POINT_ERROR),
      },
      at: {
        x: expect.closeTo(8.127_337_736_141_271e-8, FLOATING_POINT_ERROR),
        y: expect.closeTo(0.999_999_880_790_710_4, FLOATING_POINT_ERROR),
        z: expect.closeTo(-5.962_812_021_920_172e-9, FLOATING_POINT_ERROR),
        t: expect.closeTo(1.817_587_224_417_495_8e-20, FLOATING_POINT_ERROR),
      },
      transform: {
        x: expect.closeTo(-0.151_822_939_515_113_83, FLOATING_POINT_ERROR),
        y: expect.closeTo(1.036_200_165_748_596_2, FLOATING_POINT_ERROR),
        z: expect.closeTo(-0.169_958_710_670_471_2, FLOATING_POINT_ERROR),
        t: expect.closeTo(1.485_376_372_184_306e-43, FLOATING_POINT_ERROR),
      },
    });
  });
});
