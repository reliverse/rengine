/**
 * Centralized defaults for the Rengine game engine
 * This file contains all default values used throughout the application
 */

// ============================================================================
// SETTINGS DEFAULTS
// ============================================================================

/**
 * Default precision for numeric values (decimal places)
 * Controls how many decimal places are shown in UI and stored in transforms
 */
export const DEFAULT_PRECISION = 4;

/**
 * Precision range limits
 */
export const PRECISION_RANGE = {
  MIN: 0,
  MAX: 10,
} as const;

// ============================================================================
// SCENE DEFAULTS
// ============================================================================

/**
 * Default camera settings
 */
export const CAMERA_DEFAULTS = {
  POSITION: [5, 5, 5] as [number, number, number],
  TARGET: [0, 0, 0] as [number, number, number],
} as const;

/**
 * Default grid settings
 */
export const GRID_DEFAULTS = {
  VISIBLE: true,
  SNAP_TO_GRID: false,
  SIZE: 1,
} as const;

/**
 * Default scene metadata
 */
export const SCENE_METADATA_DEFAULTS = {
  name: "Untitled Scene",
  description: undefined,
  isModified: false,
  lastSavedAt: undefined,
};

// ============================================================================
// OBJECT DEFAULTS
// ============================================================================

/**
 * Default transform values for new objects
 */
export const OBJECT_DEFAULTS = {
  POSITION: [0, 0, 0] as [number, number, number],
  ROTATION: [0, 0, 0] as [number, number, number], // degrees
  SCALE: [1, 1, 1] as [number, number, number],
  COLOR: "#ffffff",
  VISIBLE: true,
} as const;

/**
 * Default object types available
 */
export const OBJECT_TYPES = {
  CUBE: "cube",
  SPHERE: "sphere",
  PLANE: "plane",
  IMPORTED: "imported",
} as const;

// ============================================================================
// LIGHT DEFAULTS
// ============================================================================

/**
 * Base light properties shared across all light types
 * Note: Intensity values are multiplied by Math.PI to account for Three.js r155+ changes
 */
export const LIGHT_BASE_DEFAULTS = {
  COLOR: "#ffffff",
  INTENSITY: Math.PI, // Adjusted for Three.js r155+ lighting changes
  VISIBLE: true,
  CAST_SHADOW: true,
} as const;

/**
 * Default properties for directional lights
 */
export const DIRECTIONAL_LIGHT_DEFAULTS = {
  ...LIGHT_BASE_DEFAULTS,
  POSITION: [10, 10, 5] as [number, number, number],
  TARGET: [0, 0, 0] as [number, number, number],
  SHADOW_BIAS: -0.0001,
  SHADOW_MAP_SIZE: 2048,
  SHADOW_NEAR: 0.1,
  SHADOW_FAR: 100,
  SHADOW_RADIUS: 8,
} as const;

/**
 * Default properties for point lights
 */
export const POINT_LIGHT_DEFAULTS = {
  ...LIGHT_BASE_DEFAULTS,
  DISTANCE: 0, // 0 = infinite
  DECAY: 2, // realistic
  SHADOW_BIAS: 0.0001,
  SHADOW_MAP_SIZE: 1024,
  SHADOW_NEAR: 0.1,
  SHADOW_FAR: 50,
  SHADOW_RADIUS: 4,
} as const;

/**
 * Default properties for spot lights
 */
export const SPOT_LIGHT_DEFAULTS = {
  ...LIGHT_BASE_DEFAULTS,
  TARGET: [0, 0, 0] as [number, number, number],
  ANGLE: Math.PI / 6, // radians (30 degrees)
  PENUMBRA: 0.1,
  DISTANCE: 0, // 0 = infinite
  DECAY: 2,
  SHADOW_BIAS: 0.0001,
  SHADOW_MAP_SIZE: 1024,
  SHADOW_NEAR: 0.1,
  SHADOW_FAR: 50,
  SHADOW_RADIUS: 4,
} as const;

/**
 * Default properties for ambient lights
 */
export const AMBIENT_LIGHT_DEFAULTS = {
  ...LIGHT_BASE_DEFAULTS,
  INTENSITY: 0.6 * Math.PI, // Adjusted for Three.js r155+ lighting changes
  CAST_SHADOW: false, // Ambient lights don't cast shadows
} as const;

/**
 * Default properties for hemisphere lights
 */
export const HEMISPHERE_LIGHT_DEFAULTS = {
  ...LIGHT_BASE_DEFAULTS,
  GROUND_COLOR: "#444444",
  INTENSITY: 0.6 * Math.PI, // Adjusted for Three.js r155+ lighting changes
  CAST_SHADOW: false, // Hemisphere lights don't cast shadows
} as const;

// ============================================================================
// UI DEFAULTS
// ============================================================================

/**
 * Default UI states
 */
export const UI_DEFAULTS = {
  LIGHTS_VISIBLE: true,
  TRANSFORM_DRAGGING: false,
  PERFORMANCE_REGRESSION_ON_MOVE: false, // Disable performance regression when moving camera by default
} as const;

/**
 * Default tool selection
 */
export const TOOL_DEFAULTS = {
  ACTIVE_TOOL: "select" as const,
} as const;

// ============================================================================
// PLACEMENT DEFAULTS
// ============================================================================

/**
 * Default placement mode settings
 */
export const PLACEMENT_DEFAULTS = {
  active: false,
  objectType: null,
  previewPosition: null,
};

// ============================================================================
// EXPORT DEFAULTS
// ============================================================================

/**
 * Default export settings
 */
export const EXPORT_DEFAULTS = {
  STREAM_DISTANCE: 200.0,
  DRAW_DISTANCE: 150.0,
  WORLD_ID: -1,
  INTERIOR_ID: -1,
  PLAYER_ID: -1,
} as const;

// ============================================================================
// CONVENIENCE EXPORTS
// ============================================================================

/**
 * All default values grouped by category
 */
export const DEFAULTS = {
  // Settings
  precision: DEFAULT_PRECISION,
  precisionRange: PRECISION_RANGE,

  // Scene
  camera: CAMERA_DEFAULTS,
  grid: GRID_DEFAULTS,
  sceneMetadata: SCENE_METADATA_DEFAULTS,
  ui: UI_DEFAULTS,
  tool: TOOL_DEFAULTS,
  placement: PLACEMENT_DEFAULTS,

  // Objects
  object: OBJECT_DEFAULTS,
  objectTypes: OBJECT_TYPES,

  // Lights
  lightBase: LIGHT_BASE_DEFAULTS,
  directionalLight: DIRECTIONAL_LIGHT_DEFAULTS,
  pointLight: POINT_LIGHT_DEFAULTS,
  spotLight: SPOT_LIGHT_DEFAULTS,
  ambientLight: AMBIENT_LIGHT_DEFAULTS,
  hemisphereLight: HEMISPHERE_LIGHT_DEFAULTS,

  // Export
  export: EXPORT_DEFAULTS,
} as const;
