declare module "gltf-validator" {
  export interface ValidationReport {
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

  export function validateBytes(data: Uint8Array): Promise<ValidationReport>;
}
