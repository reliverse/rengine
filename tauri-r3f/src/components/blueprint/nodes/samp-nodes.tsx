/**
 * SAMP-specific node components
 * Callbacks and native function nodes
 */

import type { BlueprintNode } from "~/utils/blueprint/graph/node-types";
import {
  createSampCallbackNode,
  createSampNativeNode,
} from "~/utils/blueprint/languages/pawn/pawn-nodes";
import { SAMP_CALLBACKS } from "~/utils/blueprint/languages/pawn/pawn-callbacks";
import { SAMP_NATIVES } from "~/utils/blueprint/languages/pawn/pawn-functions";

/**
 * Create a SAMP callback node from callback name
 */
export function createSampCallbackNodeByName(
  callbackName: string,
  position: { x: number; y: number }
): BlueprintNode | null {
  const callback = SAMP_CALLBACKS.find((cb) => cb.name === callbackName);
  if (!callback) {
    return null;
  }
  return createSampCallbackNode(callback, position);
}

/**
 * Create a SAMP native function node from function name
 */
export function createSampNativeNodeByName(
  nativeName: string,
  position: { x: number; y: number }
): BlueprintNode | null {
  const native = SAMP_NATIVES.find((fn) => fn.name === nativeName);
  if (!native) {
    return null;
  }
  return createSampNativeNode(native, position);
}

/**
 * Get all available SAMP callback names
 */
export function getAvailableSampCallbacks(): string[] {
  return SAMP_CALLBACKS.map((cb) => cb.name);
}

/**
 * Get all available SAMP native function names
 */
export function getAvailableSampNatives(): string[] {
  return SAMP_NATIVES.map((fn) => fn.name);
}

/**
 * Get SAMP natives by category
 */
export function getSampNativesByCategory(category: string): string[] {
  return SAMP_NATIVES.filter((fn) => fn.category === category).map(
    (fn) => fn.name
  );
}
