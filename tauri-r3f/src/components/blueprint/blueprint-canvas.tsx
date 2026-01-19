/**
 * Blueprint Canvas Component
 * Custom HTML5 Canvas-based node editor for visual scripting
 */

import { useEffect, useRef, useState, useCallback } from "react";
import { Trash2, Copy } from "lucide-react";
import type {
  BlueprintGraph,
  BlueprintNode,
  Pin,
} from "~/utils/blueprint/graph/node-types";
import { cn } from "~/lib/utils";
import { Button } from "~/components/ui/button";

interface BlueprintCanvasProps {
  graph: BlueprintGraph;
  onNodeMove?: (nodeId: string, position: { x: number; y: number }) => void;
  onNodeSelect?: (nodeId: string | null) => void;
  onConnectionStart?: (nodeId: string, pinId: string) => void;
  onConnectionEnd?: (nodeId: string, pinId: string) => void;
  onNodeDelete?: (nodeId: string) => void;
  onNodeDuplicate?: (nodeId: string) => void;
  className?: string;
}

const NODE_WIDTH = 200;
const NODE_HEADER_HEIGHT = 32;
const PIN_HEIGHT = 24;
const PIN_RADIUS = 6;
const GRID_SIZE = 20;

export function BlueprintCanvas({
  graph,
  onNodeMove,
  onNodeSelect,
  onConnectionStart,
  onConnectionEnd,
  onNodeDelete,
  onNodeDuplicate,
  className,
}: BlueprintCanvasProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [viewport, setViewport] = useState({ x: 0, y: 0, zoom: 1 });
  const [dragging, setDragging] = useState<{
    nodeId: string;
    offset: { x: number; y: number };
  } | null>(null);
  const [panning, setPanning] = useState<{
    startX: number;
    startY: number;
    startViewportX: number;
    startViewportY: number;
  } | null>(null);
  const [connecting, setConnecting] = useState<{
    nodeId: string;
    pinId: string;
    x: number;
    y: number;
    isOutput: boolean;
  } | null>(null);
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null);
  const [selectedNodeIds, setSelectedNodeIds] = useState<Set<string>>(
    new Set()
  );
  const [dragSelection, setDragSelection] = useState<{
    startX: number;
    startY: number;
    endX: number;
    endY: number;
  } | null>(null);
  const [contextMenu, setContextMenu] = useState<{
    x: number;
    y: number;
    nodeId: string | null;
  } | null>(null);

  // Convert world coordinates to screen coordinates
  const worldToScreen = useCallback(
    (world: { x: number; y: number }) => ({
      x: (world.x + viewport.x) * viewport.zoom,
      y: (world.y + viewport.y) * viewport.zoom,
    }),
    [viewport]
  );

  // Convert screen coordinates to world coordinates
  const screenToWorld = useCallback(
    (screen: { x: number; y: number }) => ({
      x: screen.x / viewport.zoom - viewport.x,
      y: screen.y / viewport.zoom - viewport.y,
    }),
    [viewport]
  );

  // Draw grid
  const drawGrid = useCallback(
    (ctx: CanvasRenderingContext2D, width: number, height: number) => {
      ctx.strokeStyle = "rgba(255, 255, 255, 0.1)";
      ctx.lineWidth = 1;

      const startX = -viewport.x % GRID_SIZE;
      const startY = -viewport.y % GRID_SIZE;

      for (let x = startX; x < width; x += GRID_SIZE * viewport.zoom) {
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, height);
        ctx.stroke();
      }

      for (let y = startY; y < height; y += GRID_SIZE * viewport.zoom) {
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(width, y);
        ctx.stroke();
      }
    },
    [viewport]
  );

  // Draw a pin
  const drawPin = useCallback(
    (
      ctx: CanvasRenderingContext2D,
      x: number,
      y: number,
      pin: Pin,
      direction: "input" | "output"
    ) => {
      const pinX = direction === "input" ? x : x;
      const pinColor =
        pin.type === "exec" ? "rgb(255, 200, 0)" : "rgb(100, 150, 255)";

      // Pin circle
      ctx.fillStyle = pin.connected ? pinColor : "rgba(60, 60, 60, 0.8)";
      ctx.strokeStyle = pinColor;
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.arc(pinX, y, PIN_RADIUS, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();

      // Pin label
      ctx.fillStyle = "rgba(255, 255, 255, 0.7)";
      ctx.font = "12px sans-serif";
      ctx.textAlign = direction === "input" ? "left" : "right";
      ctx.textBaseline = "middle";
      const labelX =
        direction === "input" ? pinX + PIN_RADIUS + 8 : pinX - PIN_RADIUS - 8;
      ctx.fillText(pin.name, labelX, y);
    },
    []
  );

  // Draw a node
  const drawNode = useCallback(
    (ctx: CanvasRenderingContext2D, node: BlueprintNode) => {
      const screenPos = worldToScreen(node.position);
      const isSelected =
        node.selected ||
        node.id === selectedNodeId ||
        selectedNodeIds.has(node.id);
      const nodeHeight =
        NODE_HEADER_HEIGHT +
        Math.max(node.inputs.length, node.outputs.length) * PIN_HEIGHT +
        16;

      // Node background
      ctx.fillStyle = isSelected
        ? "rgba(59, 130, 246, 0.2)"
        : "rgba(30, 30, 30, 0.95)";
      ctx.strokeStyle = isSelected
        ? "rgb(59, 130, 246)"
        : "rgba(100, 100, 100, 0.5)";
      ctx.lineWidth = isSelected ? 2 : 1;
      ctx.fillRect(screenPos.x, screenPos.y, NODE_WIDTH, nodeHeight);
      ctx.strokeRect(screenPos.x, screenPos.y, NODE_WIDTH, nodeHeight);

      // Node header
      ctx.fillStyle = "rgba(40, 40, 40, 0.95)";
      ctx.fillRect(screenPos.x, screenPos.y, NODE_WIDTH, NODE_HEADER_HEIGHT);
      ctx.strokeRect(screenPos.x, screenPos.y, NODE_WIDTH, NODE_HEADER_HEIGHT);

      // Node title
      ctx.fillStyle = "rgba(255, 255, 255, 0.9)";
      ctx.font = "14px sans-serif";
      ctx.textAlign = "left";
      ctx.textBaseline = "middle";
      ctx.fillText(
        node.title,
        screenPos.x + 12,
        screenPos.y + NODE_HEADER_HEIGHT / 2
      );

      // Draw input pins
      node.inputs.forEach((pin, index) => {
        const pinY =
          screenPos.y +
          NODE_HEADER_HEIGHT +
          index * PIN_HEIGHT +
          PIN_HEIGHT / 2;
        drawPin(ctx, screenPos.x, pinY, pin, "input");
      });

      // Draw output pins
      node.outputs.forEach((pin, index) => {
        const pinY =
          screenPos.y +
          NODE_HEADER_HEIGHT +
          index * PIN_HEIGHT +
          PIN_HEIGHT / 2;
        drawPin(ctx, screenPos.x + NODE_WIDTH, pinY, pin, "output");
      });
    },
    [worldToScreen, selectedNodeId, selectedNodeIds, drawPin]
  );

  // Draw connections
  const drawConnections = useCallback(
    (ctx: CanvasRenderingContext2D) => {
      ctx.strokeStyle = "rgba(100, 150, 255, 0.8)";
      ctx.lineWidth = 2;

      for (const conn of graph.connections) {
        const sourceNode = graph.nodes.find((n) => n.id === conn.sourceNodeId);
        const targetNode = graph.nodes.find((n) => n.id === conn.targetNodeId);

        if (!(sourceNode && targetNode)) continue;

        const sourcePin = [...sourceNode.outputs].find(
          (p) => p.id === conn.sourcePinId
        );
        const targetPin = [...targetNode.inputs].find(
          (p) => p.id === conn.targetPinId
        );

        if (!(sourcePin && targetPin)) continue;

        const sourcePinIndex = sourceNode.outputs.indexOf(sourcePin);
        const targetPinIndex = targetNode.inputs.indexOf(targetPin);

        const sourcePos = worldToScreen({
          x: sourceNode.position.x + NODE_WIDTH,
          y:
            sourceNode.position.y +
            NODE_HEADER_HEIGHT +
            sourcePinIndex * PIN_HEIGHT +
            PIN_HEIGHT / 2,
        });

        const targetPos = worldToScreen({
          x: targetNode.position.x,
          y:
            targetNode.position.y +
            NODE_HEADER_HEIGHT +
            targetPinIndex * PIN_HEIGHT +
            PIN_HEIGHT / 2,
        });

        // Draw bezier curve
        ctx.beginPath();
        ctx.moveTo(sourcePos.x, sourcePos.y);
        const cp1x = sourcePos.x + 50;
        const cp1y = sourcePos.y;
        const cp2x = targetPos.x - 50;
        const cp2y = targetPos.y;
        ctx.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, targetPos.x, targetPos.y);
        ctx.stroke();
      }

      // Draw connecting line if dragging
      if (connecting) {
        const node = graph.nodes.find((n) => n.id === connecting.nodeId);
        if (node) {
          const pin = [...node.inputs, ...node.outputs].find(
            (p) => p.id === connecting.pinId
          );
          if (pin) {
            const pinIndex =
              pin.direction === "input"
                ? node.inputs.indexOf(pin)
                : node.outputs.indexOf(pin);

            const nodePos = worldToScreen(node.position);
            const pinY =
              nodePos.y +
              NODE_HEADER_HEIGHT +
              pinIndex * PIN_HEIGHT +
              PIN_HEIGHT / 2;
            const pinX =
              pin.direction === "input" ? nodePos.x : nodePos.x + NODE_WIDTH;

            ctx.strokeStyle = "rgba(100, 150, 255, 0.6)";
            ctx.lineWidth = 2;
            ctx.setLineDash([5, 5]);
            ctx.beginPath();
            ctx.moveTo(pinX, pinY);
            ctx.lineTo(connecting.x, connecting.y);
            ctx.stroke();
            ctx.setLineDash([]);
          }
        }
      }
    },
    [graph, worldToScreen, connecting]
  );

  // Render function
  const render = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const rect = canvas.getBoundingClientRect();
    canvas.width = rect.width;
    canvas.height = rect.height;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Draw grid
    drawGrid(ctx, canvas.width, canvas.height);

    // Draw connections first (behind nodes)
    drawConnections(ctx);

    // Draw nodes
    for (const node of graph.nodes) {
      drawNode(ctx, node);
    }

    // Draw drag selection box
    if (dragSelection) {
      const x = Math.min(dragSelection.startX, dragSelection.endX);
      const y = Math.min(dragSelection.startY, dragSelection.endY);
      const width = Math.abs(dragSelection.endX - dragSelection.startX);
      const height = Math.abs(dragSelection.endY - dragSelection.startY);

      ctx.fillStyle = "rgba(59, 130, 246, 0.1)";
      ctx.fillRect(x, y, width, height);
      ctx.strokeStyle = "rgb(59, 130, 246)";
      ctx.lineWidth = 2;
      ctx.setLineDash([5, 5]);
      ctx.strokeRect(x, y, width, height);
      ctx.setLineDash([]);
    }
  }, [graph, drawGrid, drawNode, drawConnections, dragSelection]);

  // Handle wheel zoom
  const handleWheel = useCallback(
    (e: React.WheelEvent<HTMLCanvasElement>) => {
      e.preventDefault();
      const canvas = canvasRef.current;
      if (!canvas) return;

      const rect = canvas.getBoundingClientRect();
      const mouseX = e.clientX - rect.left;
      const mouseY = e.clientY - rect.top;

      // Zoom factor
      const zoomFactor = e.deltaY > 0 ? 0.9 : 1.1;
      const newZoom = Math.max(0.1, Math.min(3, viewport.zoom * zoomFactor));

      // Zoom towards mouse position
      const worldPos = screenToWorld({ x: mouseX, y: mouseY });
      const newViewportX = mouseX / newZoom - worldPos.x;
      const newViewportY = mouseY / newZoom - worldPos.y;

      setViewport({
        x: newViewportX,
        y: newViewportY,
        zoom: newZoom,
      });
    },
    [viewport, screenToWorld]
  );

  // Handle mouse events
  const handleMouseDown = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement>) => {
      const canvas = canvasRef.current;
      if (!canvas) return;

      const rect = canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      const worldPos = screenToWorld({ x, y });

      // Right-click for context menu
      if (e.button === 2) {
        e.preventDefault();
        let clickedNodeId: string | null = null;
        for (const node of graph.nodes) {
          const nodeHeight =
            NODE_HEADER_HEIGHT +
            Math.max(node.inputs.length, node.outputs.length) * PIN_HEIGHT +
            16;
          if (
            worldPos.x >= node.position.x &&
            worldPos.x <= node.position.x + NODE_WIDTH &&
            worldPos.y >= node.position.y &&
            worldPos.y <= node.position.y + nodeHeight
          ) {
            clickedNodeId = node.id;
            break;
          }
        }
        setContextMenu({ x: e.clientX, y: e.clientY, nodeId: clickedNodeId });
        return;
      }

      // Middle mouse button or space + left click for panning
      if (e.button === 1 || (e.button === 0 && (e.ctrlKey || e.metaKey))) {
        e.preventDefault();
        setPanning({
          startX: x,
          startY: y,
          startViewportX: viewport.x,
          startViewportY: viewport.y,
        });
        return;
      }

      // Shift + click for multi-select
      const isMultiSelect = e.shiftKey;

      // Check if clicking on a node
      let clickedOnNode = false;
      for (const node of graph.nodes) {
        const nodeHeight =
          NODE_HEADER_HEIGHT +
          Math.max(node.inputs.length, node.outputs.length) * PIN_HEIGHT +
          16;

        if (
          worldPos.x >= node.position.x &&
          worldPos.x <= node.position.x + NODE_WIDTH &&
          worldPos.y >= node.position.y &&
          worldPos.y <= node.position.y + nodeHeight
        ) {
          // Check if clicking on a pin
          const pinY = worldPos.y - node.position.y - NODE_HEADER_HEIGHT;
          const pinIndex = Math.floor(pinY / PIN_HEIGHT);

          // Check input pins
          if (
            worldPos.x <= node.position.x + PIN_RADIUS * 2 &&
            pinIndex >= 0 &&
            pinIndex < node.inputs.length
          ) {
            const pin = node.inputs[pinIndex];
            setConnecting({
              nodeId: node.id,
              pinId: pin.id,
              x,
              y,
              isOutput: false,
            });
            onConnectionStart?.(node.id, pin.id);
            return;
          }

          // Check output pins
          if (
            worldPos.x >= node.position.x + NODE_WIDTH - PIN_RADIUS * 2 &&
            pinIndex >= 0 &&
            pinIndex < node.outputs.length
          ) {
            const pin = node.outputs[pinIndex];
            setConnecting({
              nodeId: node.id,
              pinId: pin.id,
              x,
              y,
              isOutput: true,
            });
            onConnectionStart?.(node.id, pin.id);
            return;
          }

          // Clicked on node body - start dragging
          if (isMultiSelect) {
            // Multi-select: toggle selection
            const newSelected = new Set(selectedNodeIds);
            if (newSelected.has(node.id)) {
              newSelected.delete(node.id);
            } else {
              newSelected.add(node.id);
            }
            setSelectedNodeIds(newSelected);
            setSelectedNodeId(
              newSelected.size === 1 ? Array.from(newSelected)[0] : null
            );
            onNodeSelect?.(
              newSelected.size === 1 ? Array.from(newSelected)[0] : null
            );
          } else {
            // Single select
            setDragging({
              nodeId: node.id,
              offset: {
                x: worldPos.x - node.position.x,
                y: worldPos.y - node.position.y,
              },
            });
            setSelectedNodeId(node.id);
            setSelectedNodeIds(new Set([node.id]));
            onNodeSelect?.(node.id);
          }
          clickedOnNode = true;
          return;
        }
      }

      // Clicked on empty space - start drag selection
      if (!clickedOnNode && e.button === 0) {
        setDragSelection({
          startX: x,
          startY: y,
          endX: x,
          endY: y,
        });
        if (!isMultiSelect) {
          setSelectedNodeId(null);
          setSelectedNodeIds(new Set());
          onNodeSelect?.(null);
        }
      }
    },
    [
      graph,
      screenToWorld,
      onNodeSelect,
      onConnectionStart,
      viewport,
      selectedNodeIds,
    ]
  );

  const handleMouseMove = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement>) => {
      const canvas = canvasRef.current;
      if (!canvas) return;

      const rect = canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;

      // Handle panning
      if (panning) {
        const deltaX = x - panning.startX;
        const deltaY = y - panning.startY;
        setViewport({
          ...viewport,
          x: panning.startViewportX - deltaX / viewport.zoom,
          y: panning.startViewportY - deltaY / viewport.zoom,
        });
        return;
      }

      if (dragging) {
        const worldPos = screenToWorld({ x, y });
        const newPosition = {
          x: worldPos.x - dragging.offset.x,
          y: worldPos.y - dragging.offset.y,
        };
        onNodeMove?.(dragging.nodeId, newPosition);
      }

      if (connecting) {
        setConnecting({ ...connecting, x, y });
      }

      // Update drag selection box
      if (dragSelection) {
        setDragSelection({
          ...dragSelection,
          endX: x,
          endY: y,
        });
      }
    },
    [
      dragging,
      connecting,
      panning,
      viewport,
      screenToWorld,
      onNodeMove,
      dragSelection,
    ]
  );

  const handleMouseUp = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement>) => {
      // Stop panning
      if (panning) {
        setPanning(null);
        return;
      }

      // Handle drag selection completion
      if (dragSelection) {
        const canvas = canvasRef.current;
        if (!canvas) {
          setDragSelection(null);
          return;
        }

        const minX = Math.min(dragSelection.startX, dragSelection.endX);
        const minY = Math.min(dragSelection.startY, dragSelection.endY);
        const maxX = Math.max(dragSelection.startX, dragSelection.endX);
        const maxY = Math.max(dragSelection.startY, dragSelection.endY);

        // Find nodes within selection box
        const selected = new Set<string>();
        for (const node of graph.nodes) {
          const screenPos = worldToScreen(node.position);
          const nodeHeight =
            NODE_HEADER_HEIGHT +
            Math.max(node.inputs.length, node.outputs.length) * PIN_HEIGHT +
            16;

          if (
            screenPos.x + NODE_WIDTH >= minX &&
            screenPos.x <= maxX &&
            screenPos.y + nodeHeight >= minY &&
            screenPos.y <= maxY
          ) {
            selected.add(node.id);
          }
        }

        if (selected.size > 0) {
          setSelectedNodeIds(selected);
          setSelectedNodeId(
            selected.size === 1 ? Array.from(selected)[0] : null
          );
          onNodeSelect?.(selected.size === 1 ? Array.from(selected)[0] : null);
        }

        setDragSelection(null);
        return;
      }

      if (connecting) {
        const canvas = canvasRef.current;
        if (!canvas) {
          setConnecting(null);
          setDragging(null);
          return;
        }

        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        const worldPos = screenToWorld({ x, y });

        // Check if releasing on a pin
        for (const node of graph.nodes) {
          if (node.id === connecting.nodeId) continue;

          const pinY = worldPos.y - node.position.y - NODE_HEADER_HEIGHT;
          const pinIndex = Math.floor(pinY / PIN_HEIGHT);

          // If connecting from output, look for input pins
          if (connecting.isOutput) {
            if (
              worldPos.x <= node.position.x + PIN_RADIUS * 2 &&
              pinIndex >= 0 &&
              pinIndex < node.inputs.length
            ) {
              const pin = node.inputs[pinIndex];
              onConnectionEnd?.(node.id, pin.id);
              setConnecting(null);
              setDragging(null);
              return;
            }
          } else if (
            // If connecting from input, look for output pins
            worldPos.x >= node.position.x + NODE_WIDTH - PIN_RADIUS * 2 &&
            pinIndex >= 0 &&
            pinIndex < node.outputs.length
          ) {
            const pin = node.outputs[pinIndex];
            onConnectionEnd?.(node.id, pin.id);
            setConnecting(null);
            setDragging(null);
            return;
          }
        }

        setConnecting(null);
      }

      setDragging(null);
    },
    [
      connecting,
      graph,
      screenToWorld,
      onConnectionEnd,
      panning,
      dragSelection,
      worldToScreen,
      onNodeSelect,
    ]
  );

  // Calculate graph bounds for minimap
  const graphBounds = useCallback(() => {
    if (graph.nodes.length === 0) {
      return { minX: 0, minY: 0, maxX: 1000, maxY: 1000 };
    }

    let minX = Number.POSITIVE_INFINITY;
    let minY = Number.POSITIVE_INFINITY;
    let maxX = Number.NEGATIVE_INFINITY;
    let maxY = Number.NEGATIVE_INFINITY;

    for (const node of graph.nodes) {
      const nodeHeight =
        NODE_HEADER_HEIGHT +
        Math.max(node.inputs.length, node.outputs.length) * PIN_HEIGHT +
        16;
      minX = Math.min(minX, node.position.x);
      minY = Math.min(minY, node.position.y);
      maxX = Math.max(maxX, node.position.x + NODE_WIDTH);
      maxY = Math.max(maxY, node.position.y + nodeHeight);
    }

    // Add padding
    const padding = 100;
    return {
      minX: minX - padding,
      minY: minY - padding,
      maxX: maxX + padding,
      maxY: maxY + padding,
    };
  }, [graph]);

  // Render minimap
  const renderMinimap = useCallback(() => {
    const canvas = canvasRef.current;
    const minimapCanvas = minimapRef.current;
    if (!(canvas && minimapCanvas)) return;

    const bounds = graphBounds();
    const graphWidth = bounds.maxX - bounds.minX;
    const graphHeight = bounds.maxY - bounds.minY;

    const minimapSize = 200;
    const scale = Math.min(minimapSize / graphWidth, minimapSize / graphHeight);

    minimapCanvas.width = minimapSize;
    minimapCanvas.height = minimapSize;
    const ctx = minimapCanvas.getContext("2d");
    if (!ctx) return;

    // Clear
    ctx.fillStyle = "rgba(20, 20, 20, 0.9)";
    ctx.fillRect(0, 0, minimapSize, minimapSize);

    // Draw nodes
    ctx.fillStyle = "rgba(100, 150, 255, 0.6)";
    for (const node of graph.nodes) {
      const x = (node.position.x - bounds.minX) * scale;
      const y = (node.position.y - bounds.minY) * scale;
      const w = NODE_WIDTH * scale;
      const h =
        (NODE_HEADER_HEIGHT +
          Math.max(node.inputs.length, node.outputs.length) * PIN_HEIGHT +
          16) *
        scale;

      if (selectedNodeIds.has(node.id) || node.id === selectedNodeId) {
        ctx.fillStyle = "rgba(59, 130, 246, 0.8)";
      } else {
        ctx.fillStyle = "rgba(100, 150, 255, 0.6)";
      }
      ctx.fillRect(x, y, w, h);
    }

    // Draw viewport rectangle
    const rect = canvas.getBoundingClientRect();
    const viewportX = (-viewport.x * viewport.zoom - bounds.minX) * scale;
    const viewportY = (-viewport.y * viewport.zoom - bounds.minY) * scale;
    const viewportW = (rect.width / viewport.zoom) * scale;
    const viewportH = (rect.height / viewport.zoom) * scale;

    ctx.strokeStyle = "rgba(255, 200, 0, 0.8)";
    ctx.lineWidth = 2;
    ctx.strokeRect(viewportX, viewportY, viewportW, viewportH);
  }, [graph, graphBounds, viewport, selectedNodeIds, selectedNodeId]);

  const minimapRef = useRef<HTMLCanvasElement>(null);

  // Render on changes
  useEffect(() => {
    render();
    renderMinimap();
  }, [render, renderMinimap]);

  // Close context menu on click outside
  useEffect(() => {
    const handleClickOutside = () => {
      if (contextMenu) {
        setContextMenu(null);
      }
    };
    document.addEventListener("click", handleClickOutside);
    return () => document.removeEventListener("click", handleClickOutside);
  }, [contextMenu]);

  // Prevent default context menu
  useEffect(() => {
    const handleContextMenu = (e: MouseEvent) => {
      if (containerRef.current?.contains(e.target as Node)) {
        e.preventDefault();
      }
    };
    document.addEventListener("contextmenu", handleContextMenu);
    return () => document.removeEventListener("contextmenu", handleContextMenu);
  }, []);

  return (
    <div
      className={cn(
        "relative h-full w-full overflow-hidden bg-background",
        className
      )}
      ref={containerRef}
    >
      <canvas
        className="absolute inset-0 cursor-crosshair"
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onWheel={handleWheel}
        ref={canvasRef}
      />
      {/* Zoom controls */}
      <div className="absolute right-4 bottom-4 flex flex-col gap-2 rounded-lg border bg-background/80 p-2 shadow-lg">
        <button
          className="flex h-8 w-8 items-center justify-center rounded hover:bg-muted"
          onClick={() =>
            setViewport({
              ...viewport,
              zoom: Math.min(3, viewport.zoom + 0.1),
            })
          }
          title="Zoom in"
          type="button"
        >
          +
        </button>
        <div className="text-center text-xs">
          {Math.round(viewport.zoom * 100)}%
        </div>
        <button
          className="flex h-8 w-8 items-center justify-center rounded hover:bg-muted"
          onClick={() =>
            setViewport({
              ...viewport,
              zoom: Math.max(0.1, viewport.zoom - 0.1),
            })
          }
          title="Zoom out"
          type="button"
        >
          −
        </button>
        <button
          className="flex h-8 w-8 items-center justify-center rounded hover:bg-muted"
          onClick={() => setViewport({ x: 0, y: 0, zoom: 1 })}
          title="Reset view"
          type="button"
        >
          ⌂
        </button>
      </div>

      {/* Minimap */}
      {graph.nodes.length > 0 && (
        <div className="absolute top-4 left-4 rounded-lg border bg-background/90 p-2 shadow-lg">
          <canvas
            className="cursor-pointer"
            onClick={(e) => {
              const canvas = canvasRef.current;
              const minimapCanvas = minimapRef.current;
              if (!(canvas && minimapCanvas)) return;

              const bounds = graphBounds();
              const rect = minimapCanvas.getBoundingClientRect();
              const x = e.clientX - rect.left;
              const y = e.clientY - rect.top;

              const graphWidth = bounds.maxX - bounds.minX;
              const graphHeight = bounds.maxY - bounds.minY;
              const scale = Math.min(200 / graphWidth, 200 / graphHeight);

              const worldX = x / scale + bounds.minX;
              const worldY = y / scale + bounds.minY;

              const canvasRect = canvas.getBoundingClientRect();
              setViewport({
                x: -worldX + canvasRect.width / 2 / viewport.zoom,
                y: -worldY + canvasRect.height / 2 / viewport.zoom,
                zoom: viewport.zoom,
              });
            }}
            ref={minimapRef}
            style={{ width: 200, height: 200 }}
          />
        </div>
      )}

      {/* Context Menu */}
      {contextMenu && (
        <div
          className="absolute z-50 rounded-lg border bg-background shadow-lg"
          onClick={(e) => e.stopPropagation()}
          onKeyDown={(e) => {
            if (e.key === "Escape") {
              setContextMenu(null);
            }
          }}
          role="menu"
          style={{
            left: contextMenu.x,
            top: contextMenu.y,
          }}
        >
          <div className="flex flex-col p-1">
            {contextMenu.nodeId && (
              <>
                <Button
                  className="justify-start"
                  onClick={() => {
                    if (contextMenu.nodeId && onNodeDuplicate) {
                      onNodeDuplicate(contextMenu.nodeId);
                    }
                    setContextMenu(null);
                  }}
                  size="sm"
                  variant="ghost"
                >
                  <Copy className="mr-2 h-4 w-4" />
                  Duplicate
                </Button>
                <Button
                  className="justify-start text-destructive"
                  onClick={() => {
                    if (contextMenu.nodeId && onNodeDelete) {
                      onNodeDelete(contextMenu.nodeId);
                    }
                    setContextMenu(null);
                  }}
                  size="sm"
                  variant="ghost"
                >
                  <Trash2 className="mr-2 h-4 w-4" />
                  Delete
                </Button>
              </>
            )}
            {selectedNodeIds.size > 1 && (
              <Button
                className="justify-start text-destructive"
                onClick={() => {
                  for (const id of selectedNodeIds) {
                    if (onNodeDelete) {
                      onNodeDelete(id);
                    }
                  }
                  setContextMenu(null);
                }}
                size="sm"
                variant="ghost"
              >
                <Trash2 className="mr-2 h-4 w-4" />
                Delete Selected ({selectedNodeIds.size})
              </Button>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
