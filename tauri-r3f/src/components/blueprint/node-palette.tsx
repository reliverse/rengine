/**
 * Node Palette Component
 * Sidebar showing available nodes that can be added to the Blueprint
 */

import { useState } from "react";
import {
  Search,
  Code,
  Variable,
  GitBranch,
  Circle,
  Square,
} from "lucide-react";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { ScrollArea } from "~/components/ui/scroll-area";
import { cn } from "~/lib/utils";
import type { BlueprintNodeType } from "~/utils/blueprint/graph/node-types";
import {
  SAMP_CALLBACKS,
  type SampCallback,
} from "~/utils/blueprint/languages/pawn/pawn-callbacks";
import {
  SAMP_NATIVES,
  getSampNativeCategories,
  type SampNativeFunction,
} from "~/utils/blueprint/languages/pawn/pawn-functions";

interface NodePaletteProps {
  onNodeSelect?: (
    nodeType: BlueprintNodeType,
    metadata?: Record<string, any>
  ) => void;
  className?: string;
}

interface NodeDefinition {
  type: BlueprintNodeType;
  title: string;
  description: string;
  category: string;
  icon: React.ComponentType<{ className?: string }>;
  color: string;
}

const NODE_DEFINITIONS: NodeDefinition[] = [
  // Flow Control
  {
    type: "if",
    title: "If",
    description: "Conditional branching",
    category: "Flow Control",
    icon: GitBranch,
    color: "text-blue-500",
  },
  {
    type: "while",
    title: "While Loop",
    description: "Loop while condition is true",
    category: "Flow Control",
    icon: Circle,
    color: "text-blue-500",
  },
  {
    type: "for",
    title: "For Loop",
    description: "Iterate over a range",
    category: "Flow Control",
    icon: Circle,
    color: "text-blue-500",
  },
  {
    type: "return",
    title: "Return",
    description: "Return a value",
    category: "Flow Control",
    icon: GitBranch,
    color: "text-blue-500",
  },
  {
    type: "break",
    title: "Break",
    description: "Break out of loop",
    category: "Flow Control",
    icon: Square,
    color: "text-blue-500",
  },
  {
    type: "continue",
    title: "Continue",
    description: "Continue to next iteration",
    category: "Flow Control",
    icon: Square,
    color: "text-blue-500",
  },
  // Variables
  {
    type: "variable",
    title: "Variable",
    description: "Get or set a variable",
    category: "Variables",
    icon: Variable,
    color: "text-green-500",
  },
  {
    type: "constant",
    title: "Constant",
    description: "Constant value",
    category: "Variables",
    icon: Variable,
    color: "text-green-500",
  },
  // Functions
  {
    type: "function",
    title: "Function",
    description: "Define a function",
    category: "Functions",
    icon: Code,
    color: "text-purple-500",
  },
  {
    type: "call",
    title: "Call Function",
    description: "Call a function",
    category: "Functions",
    icon: Code,
    color: "text-purple-500",
  },
  // SAMP-specific
  {
    type: "callback",
    title: "SAMP Callback",
    description: "SAMP server callback",
    category: "Events",
    icon: Code,
    color: "text-green-600",
  },
  {
    type: "native",
    title: "SAMP Native",
    description: "SAMP native function",
    category: "Natives",
    icon: Code,
    color: "text-purple-600",
  },
  // Literals
  {
    type: "literal",
    title: "Literal",
    description: "Constant value (number, string, etc.)",
    category: "Literals",
    icon: Circle,
    color: "text-yellow-500",
  },
  // Operators
  {
    type: "binary",
    title: "Binary Operator",
    description: "Mathematical or logical operation",
    category: "Operators",
    icon: Code,
    color: "text-orange-500",
  },
  {
    type: "unary",
    title: "Unary Operator",
    description: "Single operand operation",
    category: "Operators",
    icon: Code,
    color: "text-orange-500",
  },
  {
    type: "assignment",
    title: "Assignment",
    description: "Assign a value to a variable",
    category: "Operators",
    icon: Code,
    color: "text-orange-500",
  },
];

const CATEGORIES = [
  "Flow Control",
  "Variables",
  "Functions",
  "Events",
  "Natives",
  "Literals",
  "Operators",
];

export function NodePalette({ onNodeSelect, className }: NodePaletteProps) {
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);

  const filteredNodes = NODE_DEFINITIONS.filter((node) => {
    const matchesSearch =
      node.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      node.description.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesCategory =
      !selectedCategory || node.category === selectedCategory;
    return matchesSearch && matchesCategory;
  });

  const groupedNodes = filteredNodes.reduce(
    (acc, node) => {
      if (!acc[node.category]) {
        acc[node.category] = [];
      }
      acc[node.category].push(node);
      return acc;
    },
    {} as Record<string, NodeDefinition[]>
  );

  return (
    <div
      className={cn("flex h-full flex-col border-r bg-background", className)}
    >
      {/* Header */}
      <div className="border-b p-4">
        <h2 className="mb-2 font-semibold text-lg">Node Palette</h2>
        <div className="relative">
          <Search className="absolute top-1/2 left-2 h-4 w-4 -translate-y-1/2 transform text-muted-foreground" />
          <Input
            className="pl-8"
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search nodes..."
            value={searchQuery}
          />
        </div>
      </div>

      {/* Categories */}
      <div className="flex gap-2 overflow-x-auto border-b p-2">
        <Button
          onClick={() => setSelectedCategory(null)}
          size="sm"
          variant={selectedCategory === null ? "default" : "ghost"}
        >
          All
        </Button>
        {CATEGORIES.map((category) => (
          <Button
            key={category}
            onClick={() => setSelectedCategory(category)}
            size="sm"
            variant={selectedCategory === category ? "default" : "ghost"}
          >
            {category}
          </Button>
        ))}
      </div>

      {/* Node List */}
      <ScrollArea className="flex-1">
        <div className="space-y-4 p-2">
          {/* SAMP Callbacks */}
          {(selectedCategory === null || selectedCategory === "Events") && (
            <div>
              <h3 className="mb-2 px-2 font-medium text-muted-foreground text-sm">
                SAMP Callbacks
              </h3>
              <div className="space-y-1">
                {SAMP_CALLBACKS.map((callback: SampCallback) => (
                  <button
                    className="flex w-full items-start gap-3 rounded-md p-2 text-left transition-colors hover:bg-muted"
                    key={callback.name}
                    onClick={() =>
                      onNodeSelect?.("callback", {
                        callbackName: callback.name,
                      })
                    }
                    type="button"
                  >
                    <Code className="mt-0.5 h-5 w-5 flex-shrink-0 text-green-600" />
                    <div className="min-w-0 flex-1">
                      <div className="font-medium text-sm">{callback.name}</div>
                      <div className="line-clamp-1 text-muted-foreground text-xs">
                        {callback.description}
                      </div>
                    </div>
                  </button>
                ))}
              </div>
            </div>
          )}

          {/* SAMP Natives by Category */}
          {(selectedCategory === null || selectedCategory === "Natives") &&
            getSampNativeCategories().map((category: string) => (
              <div key={category}>
                <h3 className="mb-2 px-2 font-medium text-muted-foreground text-sm">
                  {category} Natives
                </h3>
                <div className="space-y-1">
                  {SAMP_NATIVES.filter((fn) => fn.category === category).map(
                    (native: SampNativeFunction) => (
                      <button
                        className="flex w-full items-start gap-3 rounded-md p-2 text-left transition-colors hover:bg-muted"
                        key={native.name}
                        onClick={() =>
                          onNodeSelect?.("native", { nativeName: native.name })
                        }
                        type="button"
                      >
                        <Code className="mt-0.5 h-5 w-5 flex-shrink-0 text-purple-600" />
                        <div className="min-w-0 flex-1">
                          <div className="font-medium text-sm">
                            {native.name}
                          </div>
                          <div className="line-clamp-1 text-muted-foreground text-xs">
                            {native.description}
                          </div>
                        </div>
                      </button>
                    )
                  )}
                </div>
              </div>
            ))}

          {/* Standard Nodes */}
          {Object.entries(groupedNodes).map(([category, nodes]) => (
            <div key={category}>
              <h3 className="mb-2 px-2 font-medium text-muted-foreground text-sm">
                {category}
              </h3>
              <div className="space-y-1">
                {nodes.map((node) => {
                  const Icon = node.icon;
                  return (
                    <button
                      className="flex w-full items-start gap-3 rounded-md p-2 text-left transition-colors hover:bg-muted"
                      key={node.type}
                      onClick={() => onNodeSelect?.(node.type)}
                      onContextMenu={(e) => {
                        // Right-click for special node types (callbacks, natives)
                        if (
                          node.type === "callback" ||
                          node.type === "native"
                        ) {
                          e.preventDefault();
                          // Could show a submenu here for selecting specific callback/native
                        }
                      }}
                      type="button"
                    >
                      <Icon
                        className={cn(
                          "mt-0.5 h-5 w-5 flex-shrink-0",
                          node.color
                        )}
                      />
                      <div className="min-w-0 flex-1">
                        <div className="font-medium text-sm">{node.title}</div>
                        <div className="line-clamp-1 text-muted-foreground text-xs">
                          {node.description}
                        </div>
                      </div>
                    </button>
                  );
                })}
              </div>
            </div>
          ))}
        </div>
      </ScrollArea>
    </div>
  );
}
