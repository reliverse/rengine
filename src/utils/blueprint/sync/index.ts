/**
 * Blueprint Sync Module
 * Exports all sync-related functionality
 */

// biome-ignore lint/performance/noBarrelFile: This barrel file is used for convenient imports
export { SyncManager } from "./sync-manager";
export type {
  SyncState,
  SyncConflict,
  SyncPreferences,
} from "./sync-manager";

export {
  syncCodeToBlueprint,
  syncCodeStringToBlueprint,
  watchCodeFile,
} from "./code-to-blueprint";
export type { CodeToBlueprintOptions } from "./code-to-blueprint";

export {
  syncBlueprintToCode,
  syncBlueprintGraphToCode,
  watchBlueprintFile,
} from "./blueprint-to-code";
export type { BlueprintToCodeOptions } from "./blueprint-to-code";

export { ConflictResolver } from "./conflict-resolution";
export type {
  ConflictResolutionStrategy,
  ConflictResolutionResult,
} from "./conflict-resolution";
