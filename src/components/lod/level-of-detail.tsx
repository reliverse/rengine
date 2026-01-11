import { Detailed } from "@react-three/drei";
import type { ReactNode } from "react";

// Level of Detail (LOD) component for performance optimization
// Reduces vertex count and quality for distant objects

interface LODLevel {
  distance: number;
  children: ReactNode;
}

interface LevelOfDetailProps {
  children: LODLevel[];
  hysteresis?: number; // Prevents rapid switching between LOD levels
}

/**
 * LevelOfDetail component using Drei's Detailed component
 * Automatically switches between different quality levels based on distance from camera
 *
 * @param children - Array of LOD levels with distance thresholds and content
 * @param hysteresis - Optional hysteresis value to prevent rapid switching
 *
 * @example
 * ```tsx
 * <LevelOfDetail>
 *   {[
 *     { distance: 0, children: <HighQualityModel /> },
 *     { distance: 10, children: <MediumQualityModel /> },
 *     { distance: 20, children: <LowQualityModel /> },
 *   ]}
 * </LevelOfDetail>
 * ```
 */
export function LevelOfDetail({
  children,
  hysteresis = 0,
}: LevelOfDetailProps) {
  if (children.length === 0) return null;

  // Sort by distance (closest first)
  const sortedLevels = [...children].sort((a, b) => a.distance - b.distance);

  return (
    <Detailed
      distances={sortedLevels.map((level) => level.distance)}
      hysteresis={hysteresis}
    >
      {sortedLevels.map((level) => (
        <group key={level.distance}>{level.children}</group>
      ))}
    </Detailed>
  );
}

// Utility component for simple geometry LOD
interface GeometryLODProps {
  highPoly: ReactNode;
  mediumPoly: ReactNode;
  lowPoly: ReactNode;
  distances?: [number, number]; // [mediumDistance, lowDistance]
  hysteresis?: number;
}

/**
 * GeometryLOD component for simple geometry-based level of detail
 * Switches between high, medium, and low polygon versions
 */
export function GeometryLOD({
  highPoly,
  mediumPoly,
  lowPoly,
  distances = [15, 30],
  hysteresis = 0,
}: GeometryLODProps) {
  return (
    <LevelOfDetail hysteresis={hysteresis}>
      {[
        { distance: 0, children: highPoly },
        { distance: distances[0], children: mediumPoly },
        { distance: distances[1], children: lowPoly },
      ]}
    </LevelOfDetail>
  );
}

// Utility hook for dynamic LOD distances based on performance
export function useAdaptiveLODDistances(
  baseDistances: number[],
  performanceFactor: number
) {
  // performanceFactor ranges from 0 (worst performance) to 1 (best performance)
  // Lower performance = closer LOD switching distances
  const multiplier = 0.5 + performanceFactor * 0.5; // 0.5 to 1.0

  return baseDistances.map((distance) => distance * multiplier);
}

// Performance-aware LOD component
interface AdaptiveLODProps {
  children: LODLevel[];
  performanceFactor: number; // 0 to 1, from performance monitor
  hysteresis?: number;
}

/**
 * AdaptiveLOD component that adjusts LOD distances based on current performance
 * When performance is poor, switches to lower quality earlier
 */
export function AdaptiveLOD({
  children,
  performanceFactor,
  hysteresis = 0,
}: AdaptiveLODProps) {
  const adaptiveLevels = children.map((level) => ({
    ...level,
    distance: level.distance * (0.6 + performanceFactor * 0.8), // Scale distances based on performance
  }));

  return (
    <LevelOfDetail hysteresis={hysteresis}>{adaptiveLevels}</LevelOfDetail>
  );
}

// Predefined LOD levels for common use cases
export const LOD_PRESETS = {
  // Building LOD levels
  building: [
    { distance: 0, quality: "ultra" },
    { distance: 20, quality: "high" },
    { distance: 50, quality: "medium" },
    { distance: 100, quality: "low" },
  ],

  // Character LOD levels
  character: [
    { distance: 0, quality: "ultra" },
    { distance: 10, quality: "high" },
    { distance: 25, quality: "medium" },
    { distance: 50, quality: "low" },
  ],

  // Terrain LOD levels
  terrain: [
    { distance: 0, quality: "ultra" },
    { distance: 30, quality: "high" },
    { distance: 80, quality: "medium" },
    { distance: 150, quality: "low" },
  ],

  // Vegetation LOD levels
  vegetation: [
    { distance: 0, quality: "ultra" },
    { distance: 15, quality: "high" },
    { distance: 40, quality: "medium" },
    { distance: 80, quality: "low" },
  ],
} as const;

// Hook to get LOD children based on quality level
export function useLODChildren(
  qualityLevels: Record<string, ReactNode>,
  distances: number[]
): LODLevel[] {
  return distances.map((distance, index) => {
    const quality = Object.keys(qualityLevels)[index];
    return {
      distance,
      children: qualityLevels[quality],
    };
  });
}
