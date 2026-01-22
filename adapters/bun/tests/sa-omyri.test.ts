import { describe, test, expect, beforeAll } from "bun:test";

import fs from "node:fs";
import { createHash } from "node:crypto";
import { DffConverter, ModelType } from "../src";

interface ValidationReport {
  mimeType: string;
  validatorVersion: string;
  validatedAt: string;
  issues: {
    numErrors: number;
    numWarnings: number;
    numInfos: number;
    numHints: number;
    messages: string[];
    truncated: number;
  };
  info: {
    version: string;
    generator: string;
    resources: object[];
    animationCount: number;
    materialCount: number;
    hasMorphTargets: boolean;
    hasSkins: boolean;
    hasTextures: boolean;
    hasDefaultScene: boolean;
    drawCallCount: number;
    totalVertexCount: number;
    totalTriangleCount: number;
    maxUVs: number;
    maxInfluences: number;
    maxAttributes: number;
  };
}

describe("gltf skin converting: sa-omyri", () => {
  let conversionResultBuffer: Buffer;
  let report: ValidationReport;

  beforeAll(async () => {
    // Mock console.debug to suppress output during tests
    const originalDebug = console.debug;
    console.debug = () => {};

    const dffBuffer = fs.readFileSync("./tests/assets/sa-omyri.dff");
    const txdBuffer = fs.readFileSync("./tests/assets/sa-omyri.txd");
    const dffConverter = new DffConverter(dffBuffer, txdBuffer, ModelType.SKIN);
    const dffConversionResult = await dffConverter.convertDffToGltf();
    conversionResultBuffer = await dffConversionResult.getBuffer();

    const validator = await import("gltf-validator");
    report = await validator.validateBytes(
      new Uint8Array(conversionResultBuffer)
    );

    // Restore original console.debug
    console.debug = originalDebug;
  });

  test("GLB hash matches reference", () => {
    const expectedHash =
      "61681f1ed9ae07e345a8b7be95bc02af9d38b8b1a58da14040104b6fff21004c";
    const actualHash = createHash("sha256")
      .update(conversionResultBuffer)
      .digest("hex");
    expect(actualHash).toBe(expectedHash);
  });

  test("GLB is valid", () => {
    expect(report.issues.numErrors).toBe(0);
    expect(report.issues.numWarnings).toBe(0);
    expect(report.info.animationCount).toBe(0);

    expect(report.info.hasSkins).toBeTruthy();
    expect(report.info.hasTextures).toBeTruthy();
    expect(report.info.hasDefaultScene).toBeFalsy();
    expect(report.info.drawCallCount).toBe(1);
    expect(report.info.totalVertexCount).toBe(1147);
    expect(report.info.totalTriangleCount).toBe(1241);
    expect(report.info.maxAttributes).toBe(5);
  });
});
