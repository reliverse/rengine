// TypeScript types for COL (Collision) file data

export interface Vector3 {
  x: number;
  y: number;
  z: number;
}

export interface Sphere {
  center: Vector3;
  radius: number;
}

export interface Box {
  min: Vector3;
  max: Vector3;
}

export interface Face {
  a: number;
  b: number;
  c: number;
  material: number;
  light: number;
}

export interface Vertex {
  x: number;
  y: number;
  z: number;
}

export interface Triangle {
  a: number;
  b: number;
  c: number;
}

export interface Surface {
  material: number;
  flag: number;
  brightness: number;
  light: number;
}

export interface ColModel {
  model_id: number;
  model_name: string;
  center_of_mass: Vector3;
  bounding_box: Box;
  spheres: Sphere[];
  boxes: Box[];
  vertices: Vertex[];
  faces: Face[];
  suspension_lines: [Vector3, Vector3][];
}

export const ColVersion = {
  Col1: "COL1",
  Col2: "COL2",
  Col3: "COL3",
  Col4: "COL4",
} as const;

export type ColVersion = (typeof ColVersion)[keyof typeof ColVersion];

export interface ColFile {
  version: ColVersion;
  file_path: string;
  models: ColModel[];
}

export interface ColStatistics {
  version: string;
  model_count: number;
  total_spheres: number;
  total_boxes: number;
  total_vertices: number;
  total_faces: number;
  total_suspension_lines: number;
  file_path: string;
}

// Utility functions for COL data
export function getColVersionDisplayName(version: ColVersion): string {
  switch (version) {
    case ColVersion.Col1:
      return "COL1 (GTA III/VC)";
    case ColVersion.Col2:
      return "COL2 (GTA SA)";
    case ColVersion.Col3:
      return "COL3 (GTA SA Advanced)";
    case ColVersion.Col4:
      return "COL4 (Extended)";
    default:
      return "Unknown";
  }
}

export function getColVersionFromFourcc(fourcc: string): ColVersion | null {
  switch (fourcc) {
    case "COLL":
      return ColVersion.Col1;
    case "COL2":
      return ColVersion.Col2;
    case "COL3":
      return ColVersion.Col3;
    case "COL4":
      return ColVersion.Col4;
    default:
      return null;
  }
}
