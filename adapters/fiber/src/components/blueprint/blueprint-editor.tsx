/**
 * Blueprint Editor Component
 * Main component that ties together the canvas, palette, and property panel
 */

import { useState, useCallback } from "react";
import { Save, FolderOpen, FilePlus, Code2 } from "lucide-react";
import { Button } from "~/components/ui/button";
import { BlueprintCanvas } from "./blueprint-canvas";
import { NodePalette } from "./node-palette";
import { PropertyPanel } from "./property-panel";
import type {
  BlueprintGraph,
  BlueprintNode,
} from "~/utils/blueprint/graph/node-types";
import type { BlueprintNodeType } from "~/utils/blueprint/graph/node-types";
import {
  createBlueprintGraph,
  addNode,
  updateNode,
  moveNode,
  addConnection,
  selectNodes,
  clearSelection,
  removeNode,
  duplicateNodes,
} from "~/utils/blueprint/graph/blueprint-graph";
import {
  createBlueprintNode,
  createExecPin,
  createDataPin,
} from "~/utils/blueprint/graph/node-types";
import {
  createIfNode,
  createWhileNode,
  createForNode,
} from "./nodes/control-flow-nodes";
import {
  createSampCallbackNodeByName,
  createSampNativeNodeByName,
} from "./nodes/samp-nodes";
import {
  createBinaryOperatorNode,
  createComparisonNode,
  createLogicalOperatorNode,
  createMathOperatorNode,
  createLiteralNode,
  createVariableGetterNode,
  createVariableSetterNode,
} from "~/utils/blueprint/languages/pawn/pawn-nodes";
import { validateConnection } from "~/utils/blueprint/graph/connection-system";
import {
  openBlueprintFile,
  saveBlueprintFile,
  createNewBlueprint,
} from "~/utils/blueprint/file-management";
import { cn } from "~/lib/utils";
import { useBlueprintStore } from "~/stores/blueprint-store";
import { useSceneStore } from "~/stores/scene-store";
import { useToast } from "~/hooks/use-toast";

interface BlueprintEditorProps {
  initialGraph?: BlueprintGraph;
  onGraphChange?: (graph: BlueprintGraph) => void;
  className?: string;
}

export function BlueprintEditor({
  initialGraph,
  onGraphChange,
  className,
}: BlueprintEditorProps) {
  const {
    currentGraph,
    setCurrentGraph,
    markModified,
    currentFilePath,
    setCurrentFilePath,
    markSaved,
    isModified,
  } = useBlueprintStore();
  const { toast } = useToast();
  const { addBlueprintFile, updateBlueprintFile } = useSceneStore();
  const [graph, setGraph] = useState<BlueprintGraph>(
    () =>
      initialGraph ||
      currentGraph ||
      createBlueprintGraph("Untitled Blueprint", "pawn")
  );
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null);
  const [showPropertyPanel, setShowPropertyPanel] = useState(true);
  const [connectingFrom, setConnectingFrom] = useState<{
    nodeId: string;
    pinId: string;
  } | null>(null);

  const selectedNode = graph.nodes.find((n) => n.id === selectedNodeId) || null;

  const handleGraphChange = useCallback(
    (newGraph: BlueprintGraph) => {
      setGraph(newGraph);
      setCurrentGraph(newGraph);
      markModified();

      // Update Blueprint file in scene store if it exists
      const { blueprintFiles, updateBlueprintFile } = useSceneStore.getState();
      const existingFile = blueprintFiles.find(
        (bp) => bp.name === newGraph.name
      );
      if (existingFile) {
        updateBlueprintFile(existingFile.id, {
          isModified: true,
        });
      }

      onGraphChange?.(newGraph);
    },
    [onGraphChange, setCurrentGraph, markModified]
  );

  const handleNodeSelect = useCallback(
    (nodeId: string | null) => {
      setSelectedNodeId(nodeId);
      if (nodeId) {
        const updatedGraph = selectNodes(graph, [nodeId]);
        handleGraphChange(updatedGraph);
      } else {
        const updatedGraph = clearSelection(graph);
        handleGraphChange(updatedGraph);
      }
    },
    [graph, handleGraphChange]
  );

  const handleNodeMove = useCallback(
    (nodeId: string, position: { x: number; y: number }) => {
      const updatedGraph = moveNode(graph, nodeId, position);
      handleGraphChange(updatedGraph);
    },
    [graph, handleGraphChange]
  );

  const handleNodeUpdate = useCallback(
    (nodeId: string, updates: Partial<BlueprintNode>) => {
      const updatedGraph = updateNode(graph, nodeId, updates);
      handleGraphChange(updatedGraph);
    },
    [graph, handleGraphChange]
  );

  const handleNodeDelete = useCallback(
    (nodeId: string) => {
      const updatedGraph = removeNode(graph, nodeId);
      handleGraphChange(updatedGraph);
      if (selectedNodeId === nodeId) {
        setSelectedNodeId(null);
      }
      toast({
        title: "Node deleted",
        description: "The node has been removed from the graph.",
        duration: 2000,
      });
    },
    [graph, handleGraphChange, selectedNodeId, toast]
  );

  const handleNodeDuplicate = useCallback(
    (nodeId: string) => {
      const updatedGraph = duplicateNodes(graph, [nodeId]);
      handleGraphChange(updatedGraph);
      toast({
        title: "Node duplicated",
        description: "The node has been duplicated.",
        duration: 2000,
      });
    },
    [graph, handleGraphChange, toast]
  );

  const handleNodeAdd = useCallback(
    (nodeType: BlueprintNodeType, metadata?: Record<string, any>) => {
      // Create a new node based on type
      const position = {
        x: 400 + Math.random() * 200,
        y: 200 + Math.random() * 200,
      };

      let node: BlueprintNode;

      // Handle special node types
      if (nodeType === "callback" && metadata?.callbackName) {
        const callbackNode = createSampCallbackNodeByName(
          metadata.callbackName,
          position
        );
        if (callbackNode) {
          node = callbackNode;
        } else {
          node = createNodeFromType(nodeType, position);
        }
      } else if (nodeType === "native" && metadata?.nativeName) {
        const nativeNode = createSampNativeNodeByName(
          metadata.nativeName,
          position
        );
        if (nativeNode) {
          node = nativeNode;
        } else {
          node = createNodeFromType(nodeType, position);
        }
      } else {
        node = createNodeFromType(nodeType, position, metadata);
      }

      const updatedGraph = addNode(graph, node);
      handleGraphChange(updatedGraph);
      handleNodeSelect(node.id);
    },
    [graph, handleGraphChange, handleNodeSelect]
  );

  const handleConnectionStart = useCallback((nodeId: string, pinId: string) => {
    setConnectingFrom({ nodeId, pinId });
  }, []);

  const handleConnectionEnd = useCallback(
    (targetNodeId: string, targetPinId: string) => {
      if (!connectingFrom) return;

      const sourceNode = graph.nodes.find(
        (n) => n.id === connectingFrom.nodeId
      );
      const targetNode = graph.nodes.find((n) => n.id === targetNodeId);

      if (!(sourceNode && targetNode)) {
        setConnectingFrom(null);
        return;
      }

      const sourcePin = [...sourceNode.inputs, ...sourceNode.outputs].find(
        (p) => p.id === connectingFrom.pinId
      );
      const targetPin = [...targetNode.inputs, ...targetNode.outputs].find(
        (p) => p.id === targetPinId
      );

      if (!(sourcePin && targetPin)) {
        setConnectingFrom(null);
        return;
      }

      // Determine which is source and which is target based on pin directions
      let actualSourceNodeId = connectingFrom.nodeId;
      let actualSourcePinId = connectingFrom.pinId;
      let actualTargetNodeId = targetNodeId;
      let actualTargetPinId = targetPinId;

      // If source pin is input, swap
      if (sourcePin.direction === "input") {
        actualSourceNodeId = targetNodeId;
        actualSourcePinId = targetPinId;
        actualTargetNodeId = connectingFrom.nodeId;
        actualTargetPinId = connectingFrom.pinId;
      }

      // Validate connection
      const validation = validateConnection(
        graph,
        actualSourceNodeId,
        actualSourcePinId,
        actualTargetNodeId,
        actualTargetPinId
      );

      if (validation.valid) {
        const updatedGraph = addConnection(
          graph,
          actualSourceNodeId,
          actualSourcePinId,
          actualTargetNodeId,
          actualTargetPinId
        );
        handleGraphChange(updatedGraph);
      }

      setConnectingFrom(null);
    },
    [graph, connectingFrom, handleGraphChange]
  );

  const handleNewGraph = useCallback(async () => {
    try {
      const newGraph = createNewBlueprint("pawn");
      const blueprintId = `blueprint_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

      setGraph(newGraph);
      setCurrentGraph(newGraph);
      setCurrentFilePath(null);
      markSaved();
      setSelectedNodeId(null);

      // Add to scene store
      addBlueprintFile({
        id: blueprintId,
        name: newGraph.name,
        filePath: null,
        isModified: false,
        lastSavedAt: new Date(),
      });

      onGraphChange?.(newGraph);

      toast({
        title: "New Blueprint created",
        description: "Started a new empty Blueprint",
        duration: 2000,
      });
    } catch (error) {
      console.error("Error creating new Blueprint:", error);
      toast({
        title: "Error",
        description: "Failed to create new Blueprint",
        variant: "destructive",
        duration: 4000,
      });
    }
  }, [
    onGraphChange,
    setCurrentGraph,
    setCurrentFilePath,
    markSaved,
    addBlueprintFile,
    toast,
  ]);

  const handleSaveGraph = useCallback(async () => {
    if (!graph) {
      toast({
        title: "No Blueprint",
        description: "No Blueprint to save",
        variant: "destructive",
        duration: 2000,
      });
      return;
    }

    try {
      const savedPath = await saveBlueprintFile(
        graph,
        currentFilePath || undefined
      );
      if (savedPath) {
        setCurrentFilePath(savedPath);
        markSaved();

        // Update scene store
        const { blueprintFiles } = useSceneStore.getState();
        const existingFile = blueprintFiles.find(
          (bp) => bp.name === graph.name || bp.filePath === savedPath
        );
        if (existingFile) {
          updateBlueprintFile(existingFile.id, {
            filePath: savedPath,
            isModified: false,
            lastSavedAt: new Date(),
          });
        } else {
          addBlueprintFile({
            id: `blueprint_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
            name: graph.name,
            filePath: savedPath,
            isModified: false,
            lastSavedAt: new Date(),
          });
        }

        toast({
          title: "Blueprint saved",
          description: "Your Blueprint has been saved successfully.",
          duration: 2000,
        });
      }
    } catch (error) {
      console.error("Error saving Blueprint:", error);
      toast({
        title: "Save failed",
        description:
          error instanceof Error ? error.message : "Failed to save Blueprint",
        variant: "destructive",
        duration: 4000,
      });
    }
  }, [
    graph,
    currentFilePath,
    setCurrentFilePath,
    markSaved,
    addBlueprintFile,
    updateBlueprintFile,
    toast,
  ]);

  const handleLoadGraph = useCallback(async () => {
    try {
      const loadedGraph = await openBlueprintFile();
      if (loadedGraph) {
        setGraph(loadedGraph);
        setCurrentGraph(loadedGraph);
        setCurrentFilePath(null);
        markSaved();
        setSelectedNodeId(null);

        // Update scene store
        const { blueprintFiles } = useSceneStore.getState();
        const existingFile = blueprintFiles.find(
          (bp) => bp.name === loadedGraph.name
        );
        if (existingFile) {
          updateBlueprintFile(existingFile.id, {
            lastSavedAt: new Date(),
          });
        } else {
          addBlueprintFile({
            id: `blueprint_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
            name: loadedGraph.name,
            filePath: null,
            isModified: false,
            lastSavedAt: new Date(),
          });
        }

        onGraphChange?.(loadedGraph);

        toast({
          title: "Blueprint opened",
          description: `"${loadedGraph.name}" has been loaded successfully.`,
          duration: 2000,
        });
      }
    } catch (error) {
      console.error("Error opening Blueprint:", error);
      toast({
        title: "Open failed",
        description:
          error instanceof Error ? error.message : "Failed to open Blueprint",
        variant: "destructive",
        duration: 4000,
      });
    }
  }, [
    onGraphChange,
    setCurrentGraph,
    setCurrentFilePath,
    markSaved,
    addBlueprintFile,
    updateBlueprintFile,
    toast,
  ]);

  return (
    <div className={cn("flex h-full flex-col bg-background", className)}>
      {/* Toolbar */}
      <div className="flex items-center gap-2 border-b p-2">
        <Button onClick={handleNewGraph} size="sm" variant="outline">
          <FilePlus className="mr-2 h-4 w-4" />
          New
        </Button>
        <Button onClick={handleLoadGraph} size="sm" variant="outline">
          <FolderOpen className="mr-2 h-4 w-4" />
          Open
        </Button>
        <Button
          className={cn(
            "h-8 px-3",
            isModified && "bg-orange-500/20 hover:bg-orange-500/30"
          )}
          onClick={handleSaveGraph}
          size="sm"
          variant="outline"
        >
          <Save className="mr-2 h-4 w-4" />
          Save
          {isModified && <span className="ml-1 text-orange-500">*</span>}
        </Button>
        <div className="flex-1" />
        <div className="text-muted-foreground text-sm">
          {graph.nodes.length} nodes, {graph.connections.length} connections
        </div>
      </div>

      {/* Main Editor Area */}
      <div className="flex flex-1 overflow-hidden">
        {/* Node Palette */}
        <NodePalette className="w-64" onNodeSelect={handleNodeAdd} />

        {/* Canvas */}
        <div className="relative flex-1">
          <BlueprintCanvas
            graph={graph}
            onConnectionEnd={handleConnectionEnd}
            onConnectionStart={handleConnectionStart}
            onNodeDelete={handleNodeDelete}
            onNodeDuplicate={handleNodeDuplicate}
            onNodeMove={handleNodeMove}
            onNodeSelect={handleNodeSelect}
          />
        </div>

        {/* Property Panel */}
        {showPropertyPanel && (
          <PropertyPanel
            node={selectedNode}
            onClose={() => setShowPropertyPanel(false)}
            onUpdateNode={handleNodeUpdate}
          />
        )}

        {/* Toggle Property Panel Button */}
        {!showPropertyPanel && (
          <Button
            className="absolute top-2 right-2 z-10"
            onClick={() => setShowPropertyPanel(true)}
            size="icon"
            variant="secondary"
          >
            <Code2 className="h-4 w-4" />
          </Button>
        )}
      </div>
    </div>
  );
}

/**
 * Create a node from a node type
 */
function createNodeFromType(
  nodeType: BlueprintNodeType,
  position: { x: number; y: number },
  metadata?: Record<string, any>
): BlueprintNode {
  switch (nodeType) {
    case "if": {
      return createIfNode(position);
    }

    case "while": {
      return createWhileNode(position);
    }

    case "for": {
      return createForNode(position);
    }

    case "return": {
      return createBlueprintNode(
        "return",
        "Return",
        position,
        [
          createExecPin("input", "Exec"),
          createDataPin("input", "Value", "any", false),
        ],
        []
      );
    }

    case "variable": {
      const varName = metadata?.variableName || "Variable";
      const varType = metadata?.variableType || "any";
      if (metadata?.operation === "set") {
        return createVariableSetterNode(varName, varType, position);
      }
      return createVariableGetterNode(varName, varType, position);
    }

    case "constant": {
      const value = metadata?.value ?? 0;
      const valueType = metadata?.valueType || "int";
      return createLiteralNode(value, valueType, position);
    }

    case "literal": {
      const value = metadata?.value ?? 0;
      const valueType = metadata?.valueType || "int";
      return createLiteralNode(value, valueType, position);
    }

    case "binary": {
      const operator = metadata?.operator || "+";
      if (["==", "!=", "<", ">", "<=", ">="].includes(operator)) {
        return createComparisonNode(operator as any, position);
      }
      if (["&&", "||"].includes(operator)) {
        return createLogicalOperatorNode(operator as any, position);
      }
      if (["+", "-", "*", "/", "%"].includes(operator)) {
        return createMathOperatorNode(operator as any, position);
      }
      return createBinaryOperatorNode(operator, position);
    }

    case "assignment": {
      return createBlueprintNode(
        "assignment",
        "Assignment",
        position,
        [
          createExecPin("input", "Exec"),
          createDataPin("input", "Variable", "any", true),
          createDataPin("input", "Value", "any", true),
        ],
        [createExecPin("output", "Exec")]
      );
    }

    case "call": {
      return createBlueprintNode(
        "call",
        "Call Function",
        position,
        [
          createExecPin("input", "Exec"),
          createDataPin("input", "Function", "string", true),
        ],
        [
          createExecPin("output", "Exec"),
          createDataPin("output", "Result", "any"),
        ]
      );
    }

    default: {
      return createBlueprintNode(
        "expression",
        "Expression",
        position,
        [],
        [createDataPin("output", "Value", "any")]
      );
    }
  }
}
