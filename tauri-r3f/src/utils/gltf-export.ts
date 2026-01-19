import * as THREE from "three";
import { GLTFExporter } from "three/examples/jsm/exporters/GLTFExporter.js";
import type { SceneLight, SceneObject } from "~/stores/scene-store";

export interface ExportOptions {
  binary: boolean; // GLB vs GLTF
  includeLights: boolean;
  includeCameras: boolean;
  embedTextures: boolean;
  truncateDrawRange: boolean;
}

export interface ExportResult {
  success: boolean;
  data?: ArrayBuffer | string;
  blob?: Blob;
  error?: string;
  warnings?: string[];
}

/**
 * Export scene objects to GLTF/GLB format
 */
export class GLTFSceneExporter {
  private readonly exporter: GLTFExporter;

  constructor() {
    this.exporter = new GLTFExporter();
  }

  /**
   * Export scene objects to GLTF/GLB
   */
  async exportScene(
    objects: SceneObject[],
    lights: SceneLight[],
    options: ExportOptions = {
      binary: true,
      includeLights: true,
      includeCameras: false,
      embedTextures: true,
      truncateDrawRange: true,
    }
  ): Promise<ExportResult> {
    const result: ExportResult = {
      success: false,
      warnings: [],
    };

    try {
      // Create a root scene for export
      const exportScene = new THREE.Scene();
      exportScene.name = "Rengine Exported Scene";

      // Add objects to the scene
      for (const obj of objects) {
        if (!obj.visible) continue;

        if (obj.type === "imported" && obj.importedModel) {
          // Clone and position the imported model
          const clonedModel = obj.importedModel.clone();

          // Apply transformations
          clonedModel.position.set(...obj.position);
          clonedModel.rotation.set(
            (obj.rotation[0] * Math.PI) / 180,
            (obj.rotation[1] * Math.PI) / 180,
            (obj.rotation[2] * Math.PI) / 180
          );
          clonedModel.scale.set(...obj.scale);

          exportScene.add(clonedModel);
        } else if (
          obj.type === "cube" ||
          obj.type === "sphere" ||
          obj.type === "plane"
        ) {
          // Create geometry for basic shapes
          const geometry = this.createGeometryForObject(obj);
          if (geometry) {
            const material = new THREE.MeshStandardMaterial({
              color: new THREE.Color(obj.color),
            });

            const mesh = new THREE.Mesh(geometry, material);
            mesh.position.set(...obj.position);
            mesh.rotation.set(
              (obj.rotation[0] * Math.PI) / 180,
              (obj.rotation[1] * Math.PI) / 180,
              (obj.rotation[2] * Math.PI) / 180
            );
            mesh.scale.set(...obj.scale);
            mesh.name = obj.name;

            exportScene.add(mesh);
          }
        }
      }

      // Add lights if requested
      if (options.includeLights) {
        for (const light of lights) {
          if (!light.visible) continue;

          const threeLight = this.createLightFromSceneLight(light);
          if (threeLight) {
            exportScene.add(threeLight);
          }
        }
      }

      // Configure exporter options
      const exporterOptions = {
        binary: options.binary,
        includeCustomExtensions: true,
        embedImages: options.embedTextures,
        truncateDrawRange: options.truncateDrawRange,
      };

      // Perform the export
      const exportResult = await new Promise<any>((resolve, reject) => {
        this.exporter.parse(
          exportScene,
          (result: any) => resolve(result),
          (error: any) => reject(error),
          exporterOptions
        );
      });

      if (options.binary) {
        // GLB export returns ArrayBuffer
        result.data = exportResult as ArrayBuffer;
        result.blob = new Blob([exportResult], {
          type: "application/octet-stream",
        });
      } else {
        // GLTF export returns JSON string
        result.data = JSON.stringify(exportResult, null, 2);
        result.blob = new Blob([result.data], { type: "application/json" });
      }

      result.success = true;

      // Add warnings for unsupported features
      if (objects.some((obj) => obj.animationController)) {
        result.warnings?.push(
          "Animations are not exported (not yet supported)"
        );
      }

      if (options.includeCameras) {
        result.warnings?.push("Camera export is not yet implemented");
      }
    } catch (error) {
      result.error = `Export failed: ${error instanceof Error ? error.message : "Unknown error"}`;
    }

    return result;
  }

  /**
   * Create Three.js geometry for basic scene objects
   */
  private createGeometryForObject(
    obj: SceneObject
  ): THREE.BufferGeometry | null {
    switch (obj.type) {
      case "cube":
        return new THREE.BoxGeometry(1, 1, 1);
      case "sphere":
        return new THREE.SphereGeometry(0.5, 32, 32);
      case "plane":
        return new THREE.PlaneGeometry(10, 10);
      default:
        return null;
    }
  }

  /**
   * Create Three.js light from scene light
   */
  private createLightFromSceneLight(light: SceneLight): THREE.Light | null {
    let threeLight: THREE.Light;

    switch (light.type) {
      case "directional":
        threeLight = new THREE.DirectionalLight();
        if (light.target) {
          (threeLight as THREE.DirectionalLight).target.position.set(
            ...light.target
          );
        }
        break;

      case "point":
        threeLight = new THREE.PointLight();
        (threeLight as THREE.PointLight).distance = light.distance ?? 0;
        (threeLight as THREE.PointLight).decay = light.decay ?? 2;
        break;

      case "spot":
        threeLight = new THREE.SpotLight();
        (threeLight as THREE.SpotLight).distance = light.distance ?? 0;
        (threeLight as THREE.SpotLight).decay = light.decay ?? 2;
        // TODO: Add angle and penumbra for spot lights
        break;

      default:
        return null;
    }

    threeLight.color.setStyle(light.color);
    threeLight.intensity = light.intensity;
    threeLight.position.set(...light.position);
    threeLight.visible = light.visible;
    threeLight.castShadow = light.castShadow;

    if (light.castShadow && threeLight.shadow) {
      threeLight.shadow.mapSize.width = light.shadowMapSize ?? 512;
      threeLight.shadow.mapSize.height = light.shadowMapSize ?? 512;

      // Set near and far for shadow camera
      if (
        threeLight.shadow.camera instanceof THREE.PerspectiveCamera ||
        threeLight.shadow.camera instanceof THREE.OrthographicCamera
      ) {
        threeLight.shadow.camera.near = light.shadowNear ?? 0.1;
        threeLight.shadow.camera.far = light.shadowFar ?? 1000;
      }

      // TODO: Add shadow camera fov configuration to SceneLight interface
      if (threeLight.shadow.camera instanceof THREE.PerspectiveCamera) {
        // Default fov for shadow camera
        threeLight.shadow.camera.fov = 50;
      }
    }

    threeLight.name = light.name;
    return threeLight;
  }
}

/**
 * Convenience function to export current scene
 */
export async function exportSceneToGLTF(
  objects: SceneObject[],
  lights: SceneLight[],
  options?: Partial<ExportOptions>
): Promise<ExportResult> {
  const exporter = new GLTFSceneExporter();
  return await exporter.exportScene(objects, lights, {
    binary: true,
    includeLights: true,
    includeCameras: false,
    embedTextures: true,
    truncateDrawRange: true,
    ...options,
  });
}

/**
 * Download exported file
 */
export function downloadExportedFile(
  result: ExportResult,
  filename: string
): void {
  if (!(result.success && result.blob)) return;

  const url = URL.createObjectURL(result.blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
}
