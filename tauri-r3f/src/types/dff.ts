// TypeScript types for DFF (RenderWare) model data

export interface Vector3 {
  x: number;
  y: number;
  z: number;
}

export interface Quaternion {
  x: number;
  y: number;
  z: number;
  w: number;
}

export interface Matrix3x3 {
  right: Vector3;
  up: Vector3;
  at: Vector3;
}

export interface Frame {
  name: string;
  parent: number;
  position: Vector3;
  rotation_matrix: Matrix3x3;
  bone_data?: BoneData;
  user_data?: UserData;
}

export interface BoneData {
  bone_id: number;
  bone_index: number;
  bone_type: number;
}

export interface UserData {
  sections: UserDataSection[];
}

export interface UserDataSection {
  name: string;
  data: number[];
}

export interface UVLayer {
  u: number;
  v: number;
}

export interface Triangle {
  a: number;
  b: number;
  c: number;
}

export interface Material {
  color: Color;
  textures: Texture[];
  surface_properties: SurfaceProperties;
}

export interface Color {
  r: number;
  g: number;
  b: number;
  a: number;
}

export interface Texture {
  name: string;
  mask: string;
}

export interface SurfaceProperties {
  ambient: number;
  specular: number;
  diffuse: number;
}

export interface Geometry {
  flags: number;
  vertices: Vector3[];
  normals: Vector3[];
  uv_layers: UVLayer[][];
  triangles: Triangle[];
  materials: Material[];
  extensions: Record<string, ExtensionData>;
}

export type ExtensionData = SkinData | Record<string, any>;

export interface SkinData {
  num_bones: number;
  max_weights_per_vertex: number;
  used_bones: number;
}

export interface Atomic {
  frame: number;
  geometry: number;
  flags: number;
}

export interface DffModel {
  rw_version: number;
  frames: Frame[];
  geometries: Geometry[];
  atomics: Atomic[];
  file_path: string;
}

// Model viewer specific types
export interface ModelViewerState {
  currentModel?: DffModel;
  currentFilePath?: string;
  isLoading: boolean;
  error?: string;
  camera: {
    position: Vector3;
    target: Vector3;
    distance: number;
    azimuth: number;
    elevation: number;
  };
  selectedGeometry?: number;
  selectedFrame?: number;
  showWireframe: boolean;
  showNormals: boolean;
  showAxes: boolean;
  backgroundColor: string;
}

// Model loading types for different formats
export type ModelFormat = "dff" | "obj" | "gltf" | "glb" | "fbx";

export interface ModelLoadOptions {
  format?: ModelFormat;
  texturePath?: string;
  scale?: number;
  centerModel?: boolean;
}

export interface ModelLoadResult {
  success: boolean;
  model?: DffModel;
  error?: string;
  format?: ModelFormat;
  metadata?: {
    vertexCount: number;
    triangleCount: number;
    materialCount: number;
    hasNormals: boolean;
    hasUVs: boolean;
    bounds: {
      min: Vector3;
      max: Vector3;
      center: Vector3;
      size: Vector3;
    };
  };
}

// Camera control types
export interface CameraController {
  position: Vector3;
  target: Vector3;
  distance: number;
  azimuth: number;
  elevation: number;
  update(): void;
  setDistance(distance: number): void;
  setAzimuth(azimuth: number): void;
  setElevation(elevation: number): void;
  focusOnBounds(min: Vector3, max: Vector3): void;
  reset(): void;
}

// Axis gizmo types
export interface AxisGizmo {
  position: Vector3;
  size: number;
  visible: boolean;
}

// Model hierarchy types for inspector
export interface HierarchyNode {
  id: string;
  name: string;
  type: "frame" | "geometry" | "atomic" | "material";
  index: number;
  parent?: string;
  children: string[];
  data?: any;
  expanded?: boolean;
}

export interface ModelHierarchy {
  nodes: HierarchyNode[];
  rootNodes: string[];
}
