/**
 * Pawn-specific node components
 * Visual rendering for Pawn language nodes
 */

import type { BlueprintNode } from "~/utils/blueprint/graph/node-types";

interface PawnNodeProps {
  node: BlueprintNode;
  selected?: boolean;
}

/**
 * Render a Pawn-specific node with custom styling
 */
export function PawnNodeRenderer(_props: PawnNodeProps) {
  // This will be used for custom node rendering if needed
  // For now, the default canvas rendering is used
  return null;
}

/**
 * Get node color based on type
 */
export function getNodeColor(nodeType: string): string {
  switch (nodeType) {
    case "callback":
      return "rgb(100, 200, 100)"; // Green for callbacks
    case "native":
      return "rgb(200, 100, 200)"; // Purple for natives
    case "variable":
      return "rgb(100, 150, 255)"; // Blue for variables
    case "literal":
      return "rgb(255, 200, 100)"; // Orange for literals
    case "binary":
      return "rgb(255, 150, 100)"; // Red-orange for operators
    case "if":
    case "while":
    case "for":
      return "rgb(100, 150, 255)"; // Blue for control flow
    default:
      return "rgb(150, 150, 150)"; // Gray for others
  }
}

/**
 * Get node icon based on type
 */
export function getNodeIcon(nodeType: string): string {
  switch (nodeType) {
    case "callback":
      return "📞";
    case "native":
      return "⚙️";
    case "variable":
      return "📦";
    case "literal":
      return "🔢";
    case "binary":
      return "➕";
    case "if":
      return "❓";
    case "while":
    case "for":
      return "🔄";
    default:
      return "📄";
  }
}
