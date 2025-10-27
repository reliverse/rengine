import React from "react";

export interface Vector3 {
  x: number;
  y: number;
  z: number;
}

export interface Vector2 {
  x: number;
  y: number;
}

export interface MapObject {
  id: string;
  object_type: "cube" | "sphere" | "plane" | "custom";
  position: Vector3;
  rotation: Vector3;
  scale: Vector3;
  color: string;
  name: string;
  visible: boolean;
  metadata?: Record<string, any>;
}

export interface MapData {
  id: string;
  name: string;
  version: string;
  objects: MapObject[];
  settings: {
    gridSize: number;
    snapToGrid: boolean;
    backgroundColor: string;
  };
  createdAt: string;
  updatedAt: string;
}

export interface CameraState {
  position: Vector3;
  target: Vector3;
  zoom: number;
}

export interface EditorState {
  selectedObjectId: string | null;
  tool: "select" | "move" | "rotate" | "scale" | "add" | "mode-toggle";
  isGridVisible: boolean;
  snapToGrid: boolean;
  gridSize: number;
  camera: CameraState;
}

export interface ToolbarItem {
  id: string;
  name: string;
  icon: React.ComponentType<{ className?: string; size?: number }>;
  tool: EditorState["tool"];
  shortcut?: string;
}
