import { Detailed } from "@react-three/drei";
import { memo, useMemo } from "react";
import type * as THREE from "three";
import { useGraphicsSettings } from "~/stores/settings-store";
import { geometryCache, materialCache } from "~/utils/geometry-cache";

interface LODVariant {
  geometry?: THREE.BufferGeometry;
  material?: THREE.Material;
  children?: React.ReactNode;
  distance?: number;
}

interface LODModelProps {
  /** Array of model variants from highest to lowest quality */
  variants: LODVariant[];
  /** Distances at which to switch between LOD levels */
  distances?: number[];
  /** Whether to enable hysteresis to prevent LOD flickering */
  hysteresis?: number;
  /** Whether to use LOD switching based on distance */
  useLOD?: boolean;
  children?: React.ReactNode;
}

/**
 * Level of Detail component that automatically switches between different
 * quality versions of a model based on distance from camera.
 * Reduces vertex count and complexity for distant objects.
 */
export const LODModel = memo(function LODModel({
  variants,
  distances = [10, 20, 50],
  hysteresis = 0,
  useLOD = true,
  children,
}: LODModelProps) {
  const graphicsSettings = useGraphicsSettings();

  // Always call hooks at the top level, before any conditional logic
  // Scale distances based on graphics settings
  const scaledDistances = useMemo(() => {
    const multiplier = graphicsSettings.lodDistance;
    return distances.map((distance) => distance * multiplier);
  }, [distances, graphicsSettings.lodDistance]);

  // Create Detailed component with all variants
  const detailedChildren = useMemo(() => {
    // Ensure we have at least one variant
    if (variants.length === 0) {
      return null;
    }
    return variants.map((variant, index) => {
      // Generate stable key based on variant properties
      const key = variant.geometry?.id || `variant-${index}`;

      // If variant has geometry and material, create a mesh
      if (variant.geometry && variant.material) {
        return (
          <mesh
            geometry={variant.geometry}
            key={key}
            material={variant.material}
          />
        );
      }

      // If variant has children, render them
      if (variant.children) {
        return <group key={key}>{variant.children}</group>;
      }

      // Fallback to index-based content
      return <group key={key}>{children}</group>;
    });
  }, [variants, children]);

  // If LOD is disabled, just render the first (highest quality) variant
  if (!useLOD && variants.length > 0) {
    const variant = variants[0];
    const key = variant.geometry?.id || "variant-0";

    // If variant has geometry and material, create a mesh
    if (variant.geometry && variant.material) {
      return (
        <mesh
          geometry={variant.geometry}
          key={key}
          material={variant.material}
        />
      );
    }

    // If variant has children, render them
    if (variant.children) {
      return <group key={key}>{variant.children}</group>;
    }

    // Fallback to index-based content
    return <group key={key}>{children}</group>;
  }

  if (!detailedChildren) {
    return <>{children}</>;
  }

  return (
    <Detailed distances={scaledDistances} hysteresis={hysteresis}>
      {detailedChildren}
    </Detailed>
  );
});

// Pre-configured LOD variants for common geometry types
export interface GeometryLODProps {
  type: "cube" | "sphere" | "cylinder" | "plane";
  color: string | number;
  /** Base size for the geometry */
  size?: number;
  /** Distance thresholds for LOD switching */
  distances?: number[];
  /** Custom LOD levels (overrides automatic generation) */
  customVariants?: Array<{
    geometry: THREE.BufferGeometry;
    material: THREE.Material;
  }>;
}

/**
 * Pre-configured LOD component for basic geometries with automatic quality reduction
 */
export const GeometryLOD = memo(function GeometryLOD({
  type,
  color,
  size = 1,
  distances = [15, 30, 60],
  customVariants,
}: GeometryLODProps) {
  const graphicsSettings = useGraphicsSettings();

  const variants = useMemo(() => {
    // Use custom variants if provided
    if (customVariants && customVariants.length > 0) {
      return customVariants.map((variant) => ({
        geometry: variant.geometry,
        material: variant.material,
      }));
    }

    // Generate LOD variants based on graphics settings
    const variants: LODVariant[] = [];

    // If adaptive quality is disabled, only use one quality level
    // If enabled, use multiple LOD levels
    const useLOD = graphicsSettings.adaptiveQualityEnabled;

    switch (type) {
      case "cube":
        if (useLOD) {
          // Multiple LOD levels when adaptive quality is enabled
          variants.push({
            geometry: geometryCache.getCube(size),
            material: materialCache.getStandardMaterial(color),
          });
          variants.push({
            geometry: geometryCache.getCube(size),
            material: materialCache.getStandardMaterial(color),
          });
          variants.push({
            geometry: geometryCache.getCube(size),
            material: materialCache.getStandardMaterial(color),
          });
        } else {
          // Single quality level when adaptive quality is disabled
          variants.push({
            geometry: geometryCache.getCube(size),
            material: materialCache.getStandardMaterial(color),
          });
        }
        break;

      case "sphere": {
        // Calculate segment counts based on current quality level
        const baseSegments =
          graphicsSettings.qualityLevel === 0
            ? 32
            : // ULTRA
              graphicsSettings.qualityLevel === 1
              ? 24
              : // HIGH
                graphicsSettings.qualityLevel === 2
                ? 16
                : // MEDIUM
                  graphicsSettings.qualityLevel === 3
                  ? 12
                  : // LOW
                    8; // POTATO

        if (useLOD) {
          // Multiple LOD levels: high, medium, low quality
          const highSegments = Math.max(8, Math.floor(baseSegments * 1.5));
          const medSegments = Math.max(6, Math.floor(baseSegments * 1.0));
          const lowSegments = Math.max(4, Math.floor(baseSegments * 0.5));

          variants.push({
            geometry: geometryCache.getSphere(
              size / 2,
              highSegments,
              highSegments
            ),
            material: materialCache.getStandardMaterial(color),
          });
          variants.push({
            geometry: geometryCache.getSphere(
              size / 2,
              medSegments,
              medSegments
            ),
            material: materialCache.getStandardMaterial(color),
          });
          variants.push({
            geometry: geometryCache.getSphere(
              size / 2,
              lowSegments,
              lowSegments
            ),
            material: materialCache.getStandardMaterial(color),
          });
        } else {
          // Single quality level when adaptive quality is disabled
          variants.push({
            geometry: geometryCache.getSphere(
              size / 2,
              baseSegments,
              baseSegments
            ),
            material: materialCache.getStandardMaterial(color),
          });
        }
        break;
      }

      case "cylinder": {
        // Calculate segment counts based on current quality level
        const baseSegments =
          graphicsSettings.qualityLevel === 0
            ? 32
            : // ULTRA
              graphicsSettings.qualityLevel === 1
              ? 24
              : // HIGH
                graphicsSettings.qualityLevel === 2
                ? 16
                : // MEDIUM
                  graphicsSettings.qualityLevel === 3
                  ? 12
                  : // LOW
                    8; // POTATO

        if (useLOD) {
          // Multiple LOD levels
          const highSegments = Math.max(8, Math.floor(baseSegments * 1.5));
          const medSegments = Math.max(6, Math.floor(baseSegments * 1.0));
          const lowSegments = Math.max(4, Math.floor(baseSegments * 0.5));

          variants.push({
            geometry: geometryCache.getCylinder(
              size,
              size,
              size * 2,
              highSegments
            ),
            material: materialCache.getStandardMaterial(color),
          });
          variants.push({
            geometry: geometryCache.getCylinder(
              size,
              size,
              size * 2,
              medSegments
            ),
            material: materialCache.getStandardMaterial(color),
          });
          variants.push({
            geometry: geometryCache.getCylinder(
              size,
              size,
              size * 2,
              lowSegments
            ),
            material: materialCache.getStandardMaterial(color),
          });
        } else {
          // Single quality level
          variants.push({
            geometry: geometryCache.getCylinder(
              size,
              size,
              size * 2,
              baseSegments
            ),
            material: materialCache.getStandardMaterial(color),
          });
        }
        break;
      }

      case "plane": {
        // Planes typically don't need LOD as they're already simple
        const planeGeom = geometryCache.getPlane(size * 10, size * 10);
        if (useLOD) {
          variants.push(
            {
              geometry: planeGeom,
              material: materialCache.getStandardMaterial(color),
            },
            {
              geometry: planeGeom,
              material: materialCache.getStandardMaterial(color),
            },
            {
              geometry: planeGeom,
              material: materialCache.getStandardMaterial(color),
            }
          );
        } else {
          variants.push({
            geometry: planeGeom,
            material: materialCache.getStandardMaterial(color),
          });
        }
        break;
      }

      default:
        // Unknown geometry type - return empty array
        break;
    }

    return variants;
  }, [
    type,
    color,
    size,
    customVariants,
    graphicsSettings.adaptiveQualityEnabled,
    graphicsSettings.qualityLevel,
  ]);

  return (
    <LODModel
      distances={distances}
      useLOD={graphicsSettings.adaptiveQualityEnabled}
      variants={variants}
    />
  );
});

// Hook for creating LOD variants programmatically
export function useLODVariants(
  baseGeometry: THREE.BufferGeometry,
  baseMaterial: THREE.Material,
  qualityLevels = 3
) {
  return useMemo(() => {
    const variants: LODVariant[] = [];

    for (let i = 0; i < qualityLevels; i++) {
      // For now, just use the same geometry/material
      // In a real implementation, you might simplify the geometry
      // or use lower-resolution versions
      variants.push({
        geometry: baseGeometry,
        material: baseMaterial,
      });
    }

    return variants;
  }, [baseGeometry, baseMaterial, qualityLevels]);
}

// Utility function to simplify geometry for LOD (basic implementation)
export function simplifyGeometry(
  geometry: THREE.BufferGeometry,
  _factor = 0.5
): THREE.BufferGeometry {
  // This is a placeholder for actual geometry simplification
  // In a real implementation, you would use libraries like meshoptimizer
  // or implement edge collapse algorithms
  // factor parameter will be used when actual simplification is implemented

  // For now, just return the original geometry
  // TODO: Implement actual geometry simplification using factor
  return geometry;
}
