import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useEffect, useRef, useState } from "react";
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { Checkbox } from "~/components/ui/checkbox";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";
import { ScrollArea } from "~/components/ui/scroll-area";
import { useToast } from "~/hooks/use-toast";
import { cn } from "~/lib/utils";
import { useSelectedObjects } from "~/stores/scene-store";
import type {
  DffModel,
  HierarchyNode,
  ModelHierarchy,
  ModelViewerState,
  Vector3,
} from "~/types/dff";

// Custom Blender-style camera controller
class BlenderCameraController {
  private readonly camera: THREE.PerspectiveCamera;
  private readonly controls: OrbitControls;
  private spherical: { distance: number; azimuth: number; elevation: number };
  private readonly viewCenter: THREE.Vector3;

  constructor(camera: THREE.PerspectiveCamera, canvas: HTMLCanvasElement) {
    this.camera = camera;
    this.viewCenter = new THREE.Vector3(0, 0, 0);
    this.spherical = { distance: 8, azimuth: 35, elevation: 35 };

    this.controls = new OrbitControls(camera, canvas);
    this.controls.enableDamping = true;
    this.controls.dampingFactor = 0.05;
    this.controls.enableZoom = true;
    this.controls.enableRotate = true;
    this.controls.enablePan = true;

    this.applySphericalToCamera();
  }

  private applySphericalToCamera() {
    const { distance, azimuth, elevation } = this.spherical;
    const az = THREE.MathUtils.degToRad(azimuth);
    const el = THREE.MathUtils.degToRad(elevation);

    const x = distance * Math.cos(el) * Math.cos(az);
    const y = distance * Math.sin(el);
    const z = distance * Math.cos(el) * Math.sin(az);

    const position = new THREE.Vector3(x, y, z).add(this.viewCenter);
    this.camera.position.copy(position);
    this.controls.target.copy(this.viewCenter);
  }

  update() {
    this.controls.update();
  }

  setDistance(distance: number) {
    this.spherical.distance = Math.max(0.1, distance);
    this.applySphericalToCamera();
  }

  setAzimuth(azimuth: number) {
    this.spherical.azimuth = azimuth;
    this.applySphericalToCamera();
  }

  setElevation(elevation: number) {
    this.spherical.elevation = THREE.MathUtils.clamp(elevation, -89, 89);
    this.applySphericalToCamera();
  }

  focusOnBounds(min: Vector3, max: Vector3) {
    const center = new THREE.Vector3(
      (min.x + max.x) / 2,
      (min.y + max.y) / 2,
      (min.z + max.z) / 2
    );
    const radius = Math.max(max.x - min.x, max.y - min.y, max.z - min.z);

    this.viewCenter.copy(center);
    this.spherical.distance = Math.max(4.0, radius * 1.5);
    this.applySphericalToCamera();
  }

  reset() {
    this.viewCenter.set(0, 0, 0);
    this.spherical = { distance: 8, azimuth: 35, elevation: 35 };
    this.applySphericalToCamera();
  }

  getState() {
    return {
      position: this.camera.position.clone(),
      target: this.viewCenter.clone(),
      distance: this.spherical.distance,
      azimuth: this.spherical.azimuth,
      elevation: this.spherical.elevation,
    };
  }
}

// Axis gizmo component
function AxisGizmo({
  position = [0, 0, 0],
  size = 1,
}: {
  position?: [number, number, number];
  size?: number;
}) {
  return (
    <group position={position}>
      {/* X Axis - Red */}
      <mesh position={[size / 2, 0, 0]}>
        <cylinderGeometry args={[size * 0.02, size * 0.02, size, 8]} />
        <meshBasicMaterial color="#ff0000" />
      </mesh>
      {/* Y Axis - Green */}
      <mesh position={[0, size / 2, 0]} rotation={[0, 0, Math.PI / 2]}>
        <cylinderGeometry args={[size * 0.02, size * 0.02, size, 8]} />
        <meshBasicMaterial color="#00ff00" />
      </mesh>
      {/* Z Axis - Blue */}
      <mesh position={[0, 0, size / 2]} rotation={[Math.PI / 2, 0, 0]}>
        <cylinderGeometry args={[size * 0.02, size * 0.02, size, 8]} />
        <meshBasicMaterial color="#0000ff" />
      </mesh>
    </group>
  );
}

interface ModelViewerProps {
  className?: string;
}

export function ModelViewer({ className }: ModelViewerProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const rendererRef = useRef<THREE.WebGLRenderer | null>(null);
  const sceneRef = useRef<THREE.Scene | null>(null);
  const cameraRef = useRef<THREE.PerspectiveCamera | null>(null);
  const cameraControllerRef = useRef<BlenderCameraController | null>(null);
  const modelGroupRef = useRef<THREE.Group | null>(null);
  const animationFrameRef = useRef<number | null>(null);

  const [state, setState] = useState<ModelViewerState>({
    isLoading: false,
    camera: {
      position: { x: 5, y: 5, z: 5 },
      target: { x: 0, y: 0, z: 0 },
      distance: 8,
      azimuth: 35,
      elevation: 35,
    },
    showWireframe: false,
    showNormals: false,
    showAxes: true,
    backgroundColor: "#1a1a1a",
  });

  const [hierarchy, setHierarchy] = useState<ModelHierarchy>({
    nodes: [],
    rootNodes: [],
  });
  const selectedObjects = useSelectedObjects();
  const { toast } = useToast();

  // Initialize Three.js scene
  useEffect(() => {
    if (!canvasRef.current) return;

    const canvas = canvasRef.current;

    // Create renderer
    const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
    renderer.setSize(canvas.clientWidth, canvas.clientHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.shadowMap.enabled = true;
    renderer.shadowMap.type = THREE.PCFSoftShadowMap;
    rendererRef.current = renderer;

    // Create scene
    const scene = new THREE.Scene();
    scene.background = new THREE.Color(state.backgroundColor);
    sceneRef.current = scene;

    // Create camera
    const camera = new THREE.PerspectiveCamera(
      45,
      canvas.clientWidth / canvas.clientHeight,
      0.1,
      1000
    );
    cameraRef.current = camera;

    // Create camera controller
    const cameraController = new BlenderCameraController(camera, canvas);
    cameraControllerRef.current = cameraController;

    // Create model group
    const modelGroup = new THREE.Group();
    scene.add(modelGroup);
    modelGroupRef.current = modelGroup;

    // Add lights
    const ambientLight = new THREE.AmbientLight(0x40_40_40, 0.4);
    scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xff_ff_ff, 0.8);
    directionalLight.position.set(5, 5, 5);
    directionalLight.castShadow = true;
    directionalLight.shadow.mapSize.width = 2048;
    directionalLight.shadow.mapSize.height = 2048;
    scene.add(directionalLight);

    const fillLight = new THREE.DirectionalLight(0xa0_c0_ff, 0.3);
    fillLight.position.set(-5, 5, -5);
    scene.add(fillLight);

    // Add grid helper
    const gridHelper = new THREE.GridHelper(20, 20, 0x44_44_44, 0x22_22_22);
    scene.add(gridHelper);

    // Animation loop
    const animate = () => {
      animationFrameRef.current = requestAnimationFrame(animate);
      cameraController.update();
      renderer.render(scene, camera);
    };
    animate();

    // Handle resize
    const handleResize = () => {
      if (!(canvas && camera && renderer)) return;

      camera.aspect = canvas.clientWidth / canvas.clientHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(canvas.clientWidth, canvas.clientHeight);
    };

    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
      renderer.dispose();
    };
  }, [state.backgroundColor]);

  // Update background color
  useEffect(() => {
    if (sceneRef.current) {
      sceneRef.current.background = new THREE.Color(state.backgroundColor);
    }
  }, [state.backgroundColor]);

  // Display selected scene objects
  useEffect(() => {
    if (!(modelGroupRef.current && sceneRef.current)) return;

    // Clear existing model
    while (modelGroupRef.current.children.length > 0) {
      modelGroupRef.current.remove(modelGroupRef.current.children[0]);
    }

    if (selectedObjects.length > 0) {
      // Display selected scene objects
      for (const sceneObject of selectedObjects) {
        let mesh: THREE.Object3D;

        if (sceneObject.importedModel) {
          // Clone the imported model
          mesh = sceneObject.importedModel.clone();
          mesh.name = sceneObject.name;
        } else {
          // Create primitive geometry
          let geometry: THREE.BufferGeometry;
          switch (sceneObject.type) {
            case "cube":
              geometry = new THREE.BoxGeometry(1, 1, 1);
              break;
            case "sphere":
              geometry = new THREE.SphereGeometry(0.5, 32, 16);
              break;
            case "plane":
              geometry = new THREE.PlaneGeometry(1, 1);
              break;
            default:
              geometry = new THREE.BoxGeometry(1, 1, 1);
          }

          const material = new THREE.MeshPhongMaterial({
            color: sceneObject.color || "#c0c0c0",
            wireframe: state.showWireframe,
          });

          mesh = new THREE.Mesh(geometry, material);
          mesh.name = sceneObject.name;
        }

        // Apply transforms
        mesh.position.set(...sceneObject.position);
        mesh.rotation.set(
          THREE.MathUtils.degToRad(sceneObject.rotation[0]),
          THREE.MathUtils.degToRad(sceneObject.rotation[1]),
          THREE.MathUtils.degToRad(sceneObject.rotation[2])
        );
        mesh.scale.set(...sceneObject.scale);

        // Enable shadows
        if (mesh instanceof THREE.Mesh) {
          mesh.castShadow = true;
          mesh.receiveShadow = true;
        }

        modelGroupRef.current?.add(mesh);
      }

      // Focus camera on the selected objects
      if (cameraControllerRef.current && selectedObjects.length > 0) {
        // Calculate bounds of all selected objects
        const min = new THREE.Vector3(
          Number.POSITIVE_INFINITY,
          Number.POSITIVE_INFINITY,
          Number.POSITIVE_INFINITY
        );
        const max = new THREE.Vector3(
          Number.NEGATIVE_INFINITY,
          Number.NEGATIVE_INFINITY,
          Number.NEGATIVE_INFINITY
        );

        for (const sceneObject of selectedObjects) {
          // Create a bounding box for the object
          const box = new THREE.Box3();

          if (sceneObject.importedModel) {
            box.setFromObject(sceneObject.importedModel);
          } else {
            // For primitives, create a rough bounding box
            const size = Math.max(...sceneObject.scale);
            box.min.set(
              sceneObject.position[0] - size / 2,
              sceneObject.position[1] - size / 2,
              sceneObject.position[2] - size / 2
            );
            box.max.set(
              sceneObject.position[0] + size / 2,
              sceneObject.position[1] + size / 2,
              sceneObject.position[2] + size / 2
            );
          }

          // Apply object transforms to the bounding box
          const matrix = new THREE.Matrix4();
          matrix.makeRotationFromEuler(
            new THREE.Euler(
              THREE.MathUtils.degToRad(sceneObject.rotation[0]),
              THREE.MathUtils.degToRad(sceneObject.rotation[1]),
              THREE.MathUtils.degToRad(sceneObject.rotation[2])
            )
          );
          matrix.setPosition(...sceneObject.position);
          box.applyMatrix4(matrix);

          // Expand the total bounds
          min.min(box.min);
          max.max(box.max);
        }

        cameraControllerRef.current.focusOnBounds(
          { x: min.x, y: min.y, z: min.z },
          { x: max.x, y: max.y, z: max.z }
        );
      }
    }

    // Update wireframe state for existing objects
    modelGroupRef.current.traverse((child) => {
      if (
        child instanceof THREE.Mesh &&
        child.material instanceof THREE.Material &&
        "wireframe" in child.material
      ) {
        (child.material as THREE.MeshBasicMaterial).wireframe =
          state.showWireframe;
        child.material.needsUpdate = true;
      }
    });
  }, [selectedObjects, state.showWireframe]);

  // Load DFF model
  const loadDffModel = async (filePath: string) => {
    setState((prev) => ({ ...prev, isLoading: true, error: undefined }));

    try {
      const model: DffModel = await invoke("load_dff_model", {
        path: filePath,
      });

      // Clear existing model
      if (modelGroupRef.current) {
        while (modelGroupRef.current.children.length > 0) {
          modelGroupRef.current.remove(modelGroupRef.current.children[0]);
        }
      }

      // Create geometries from DFF data
      createGeometriesFromDff(model);

      // Update hierarchy
      createHierarchyFromDff(model);

      // Focus camera on model
      focusCameraOnModel(model);

      setState((prev) => ({
        ...prev,
        currentModel: model,
        currentFilePath: filePath,
        isLoading: false,
      }));

      toast({
        title: "Model Loaded",
        description: `Successfully loaded DFF model with ${model.frames.length} frames and ${model.geometries.length} geometries.`,
      });
    } catch (error) {
      console.error("Failed to load DFF model:", error);
      setState((prev) => ({
        ...prev,
        isLoading: false,
        error: error instanceof Error ? error.message : String(error),
      }));

      toast({
        title: "Load Failed",
        description: error instanceof Error ? error.message : String(error),
        variant: "destructive",
      });
    }
  };

  const createGeometriesFromDff = (model: DffModel) => {
    if (!modelGroupRef.current) return;

    model.geometries.forEach((geometry, geomIndex) => {
      // Create geometry
      const threeGeometry = new THREE.BufferGeometry();

      // Add vertices
      const vertices = new Float32Array(geometry.vertices.length * 3);
      geometry.vertices.forEach((vertex, i) => {
        vertices[i * 3] = vertex.x;
        vertices[i * 3 + 1] = vertex.y;
        vertices[i * 3 + 2] = vertex.z;
      });
      threeGeometry.setAttribute(
        "position",
        new THREE.BufferAttribute(vertices, 3)
      );

      // Add normals if available
      if (geometry.normals.length > 0) {
        const normals = new Float32Array(geometry.normals.length * 3);
        geometry.normals.forEach((normal, i) => {
          normals[i * 3] = normal.x;
          normals[i * 3 + 1] = normal.y;
          normals[i * 3 + 2] = normal.z;
        });
        threeGeometry.setAttribute(
          "normal",
          new THREE.BufferAttribute(normals, 3)
        );
      }

      // Add UVs if available
      if (geometry.uv_layers.length > 0 && geometry.uv_layers[0].length > 0) {
        const uvs = new Float32Array(geometry.uv_layers[0].length * 2);
        geometry.uv_layers[0].forEach((uv, i) => {
          uvs[i * 2] = uv.u;
          uvs[i * 2 + 1] = 1.0 - uv.v; // Flip V coordinate for Three.js
        });
        threeGeometry.setAttribute("uv", new THREE.BufferAttribute(uvs, 2));
      }

      // Add indices
      const indices = new Uint16Array(geometry.triangles.length * 3);
      geometry.triangles.forEach((triangle, i) => {
        indices[i * 3] = triangle.a;
        indices[i * 3 + 1] = triangle.b;
        indices[i * 3 + 2] = triangle.c;
      });
      threeGeometry.setIndex(new THREE.BufferAttribute(indices, 1));

      threeGeometry.computeBoundingBox();
      threeGeometry.computeBoundingSphere();

      // Create material
      let material: THREE.Material;
      if (geometry.materials.length > 0) {
        const dffMaterial = geometry.materials[0];
        const color = new THREE.Color(
          dffMaterial.color.r / 255,
          dffMaterial.color.g / 255,
          dffMaterial.color.b / 255
        );

        material = new THREE.MeshPhongMaterial({
          color,
          transparent: dffMaterial.color.a < 255,
          opacity: dffMaterial.color.a / 255,
          wireframe: state.showWireframe,
        });
      } else {
        material = new THREE.MeshPhongMaterial({
          color: 0xc0_c0_c0,
          wireframe: state.showWireframe,
        });
      }

      // Create mesh
      const mesh = new THREE.Mesh(threeGeometry, material);
      mesh.castShadow = true;
      mesh.receiveShadow = true;
      mesh.name = `geometry_${geomIndex}`;

      modelGroupRef.current?.add(mesh);
    });
  };

  const createHierarchyFromDff = (model: DffModel) => {
    const nodes: HierarchyNode[] = [];
    const rootNodes: string[] = [];

    // Add frame nodes
    model.frames.forEach((frame, index) => {
      const nodeId = `frame_${index}`;
      nodes.push({
        id: nodeId,
        name: frame.name || `Frame ${index}`,
        type: "frame",
        index,
        parent: frame.parent >= 0 ? `frame_${frame.parent}` : undefined,
        children: [],
        data: frame,
        expanded: false,
      });

      if (frame.parent < 0) {
        rootNodes.push(nodeId);
      }
    });

    // Add geometry nodes
    model.geometries.forEach((geometry, index) => {
      const nodeId = `geometry_${index}`;
      nodes.push({
        id: nodeId,
        name: `Geometry ${index}`,
        type: "geometry",
        index,
        children: [],
        data: geometry,
        expanded: false,
      });
      rootNodes.push(nodeId);
    });

    // Add atomic nodes
    model.atomics.forEach((atomic, index) => {
      const nodeId = `atomic_${index}`;
      nodes.push({
        id: nodeId,
        name: `Atomic ${index}`,
        type: "atomic",
        index,
        children: [],
        data: atomic,
        expanded: false,
      });
      rootNodes.push(nodeId);
    });

    // Build parent-child relationships
    for (const node of nodes) {
      if (node.parent) {
        const parentNode = nodes.find((n) => n.id === node.parent);
        if (parentNode) {
          parentNode.children.push(node.id);
        }
      }
    }

    setHierarchy({ nodes, rootNodes });
  };

  const focusCameraOnModel = (model: DffModel) => {
    if (!cameraControllerRef.current || model.geometries.length === 0) return;

    // Calculate bounds
    const min = new THREE.Vector3(
      Number.POSITIVE_INFINITY,
      Number.POSITIVE_INFINITY,
      Number.POSITIVE_INFINITY
    );
    const max = new THREE.Vector3(
      Number.NEGATIVE_INFINITY,
      Number.NEGATIVE_INFINITY,
      Number.NEGATIVE_INFINITY
    );

    for (const geometry of model.geometries) {
      for (const vertex of geometry.vertices) {
        min.x = Math.min(min.x, vertex.x);
        min.y = Math.min(min.y, vertex.y);
        min.z = Math.min(min.z, vertex.z);
        max.x = Math.max(max.x, vertex.x);
        max.y = Math.max(max.y, vertex.y);
        max.z = Math.max(max.z, vertex.z);
      }
    }

    cameraControllerRef.current.focusOnBounds(
      { x: min.x, y: min.y, z: min.z },
      { x: max.x, y: max.y, z: max.z }
    );
  };

  const handleOpenFile = async () => {
    try {
      const selected = await open({
        filters: [
          {
            name: "3D Models",
            extensions: ["dff", "obj", "gltf", "glb", "fbx"],
          },
        ],
      });

      if (selected && typeof selected === "string") {
        const extension = selected.split(".").pop()?.toLowerCase();
        if (extension === "dff") {
          await loadDffModel(selected);
        } else {
          toast({
            title: "Unsupported Format",
            description: `Loading ${extension?.toUpperCase()} files is not yet implemented.`,
            variant: "destructive",
          });
        }
      }
    } catch (error) {
      console.error("Failed to open file:", error);
    }
  };

  const handleResetView = () => {
    cameraControllerRef.current?.reset();
  };

  const renderHierarchyNode = (nodeId: string, depth = 0): React.ReactNode => {
    const node = hierarchy.nodes.find((n) => n.id === nodeId);
    if (!node) return null;

    const children = node.children.map((childId) =>
      renderHierarchyNode(childId, depth + 1)
    );

    return (
      <div key={node.id} style={{ marginLeft: `${depth * 16}px` }}>
        <div className="cursor-pointer rounded px-2 py-1 text-sm hover:bg-muted/50">
          {node.name} ({node.type})
        </div>
        {children}
      </div>
    );
  };

  return (
    <div className={cn("flex h-full", className)}>
      {/* Main 3D Viewport */}
      <div className="relative flex-1">
        <canvas
          className="h-full w-full"
          ref={canvasRef}
          style={{ cursor: "grab" }}
        />

        {/* Loading overlay */}
        {state.isLoading && (
          <div className="absolute inset-0 flex items-center justify-center bg-black/50">
            <div className="text-lg text-white">Loading model...</div>
          </div>
        )}

        {/* Error overlay */}
        {state.error && (
          <div className="absolute inset-0 flex items-center justify-center bg-red-900/50 p-4">
            <div className="max-w-md rounded border border-red-400 bg-red-100 px-4 py-3 text-red-700">
              <strong className="font-bold">Error loading model:</strong>
              <div className="mt-2">{state.error}</div>
            </div>
          </div>
        )}

        {/* Axis Gizmo Toggle */}
        {state.showAxes && (
          <div className="absolute top-4 right-4">
            <AxisGizmo size={1} />
          </div>
        )}
      </div>

      {/* Control Panel */}
      <div className="flex w-80 flex-col border-l bg-muted/30">
        {/* Toolbar */}
        <div className="space-y-2 border-b p-4">
          <div className="flex gap-2">
            <Button className="flex-1" onClick={handleOpenFile}>
              Open Model
            </Button>
            <Button onClick={handleResetView} variant="outline">
              Reset View
            </Button>
          </div>

          {state.currentModel && selectedObjects.length === 0 && (
            <div className="text-muted-foreground text-sm">
              <div>Frames: {state.currentModel.frames.length}</div>
              <div>Geometries: {state.currentModel.geometries.length}</div>
              <div>Atomics: {state.currentModel.atomics.length}</div>
            </div>
          )}

          {selectedObjects.length > 0 && (
            <div className="text-muted-foreground text-sm">
              <div>Selected Objects: {selectedObjects.length}</div>
              <div className="truncate">
                {selectedObjects.map((obj) => obj.name).join(", ")}
              </div>
            </div>
          )}
        </div>

        <ScrollArea className="flex-1">
          <div className="space-y-4 p-4">
            {/* Display Options */}
            <Card>
              <CardHeader>
                <CardTitle className="text-sm">Display Options</CardTitle>
              </CardHeader>
              <CardContent className="space-y-3">
                <div className="flex items-center space-x-2">
                  <Checkbox
                    checked={state.showWireframe}
                    id="wireframe"
                    onCheckedChange={(checked) =>
                      setState((prev) => ({
                        ...prev,
                        showWireframe: checked as boolean,
                      }))
                    }
                  />
                  <Label className="text-sm" htmlFor="wireframe">
                    Wireframe
                  </Label>
                </div>

                <div className="flex items-center space-x-2">
                  <Checkbox
                    checked={state.showNormals}
                    id="normals"
                    onCheckedChange={(checked) =>
                      setState((prev) => ({
                        ...prev,
                        showNormals: checked as boolean,
                      }))
                    }
                  />
                  <Label className="text-sm" htmlFor="normals">
                    Show Normals
                  </Label>
                </div>

                <div className="flex items-center space-x-2">
                  <Checkbox
                    checked={state.showAxes}
                    id="axes"
                    onCheckedChange={(checked) =>
                      setState((prev) => ({
                        ...prev,
                        showAxes: checked as boolean,
                      }))
                    }
                  />
                  <Label className="text-sm" htmlFor="axes">
                    Show Axes
                  </Label>
                </div>
              </CardContent>
            </Card>

            {/* Camera Controls */}
            <Card>
              <CardHeader>
                <CardTitle className="text-sm">Camera</CardTitle>
              </CardHeader>
              <CardContent className="space-y-3">
                <div>
                  <Label className="text-xs" htmlFor="distance">
                    Distance
                  </Label>
                  <Input
                    className="h-8 text-xs"
                    id="distance"
                    onChange={(e) => {
                      const distance = Number.parseFloat(e.target.value);
                      setState((prev) => ({
                        ...prev,
                        camera: { ...prev.camera, distance },
                      }));
                      cameraControllerRef.current?.setDistance(distance);
                    }}
                    type="number"
                    value={state.camera.distance}
                  />
                </div>

                <div>
                  <Label className="text-xs" htmlFor="azimuth">
                    Azimuth
                  </Label>
                  <Input
                    className="h-8 text-xs"
                    id="azimuth"
                    onChange={(e) => {
                      const azimuth = Number.parseFloat(e.target.value);
                      setState((prev) => ({
                        ...prev,
                        camera: { ...prev.camera, azimuth },
                      }));
                      cameraControllerRef.current?.setAzimuth(azimuth);
                    }}
                    type="number"
                    value={state.camera.azimuth}
                  />
                </div>

                <div>
                  <Label className="text-xs" htmlFor="elevation">
                    Elevation
                  </Label>
                  <Input
                    className="h-8 text-xs"
                    id="elevation"
                    onChange={(e) => {
                      const elevation = Number.parseFloat(e.target.value);
                      setState((prev) => ({
                        ...prev,
                        camera: { ...prev.camera, elevation },
                      }));
                      cameraControllerRef.current?.setElevation(elevation);
                    }}
                    type="number"
                    value={state.camera.elevation}
                  />
                </div>
              </CardContent>
            </Card>

            {/* Model Hierarchy or Scene Objects */}
            {selectedObjects.length > 0 ? (
              <Card>
                <CardHeader>
                  <CardTitle className="text-sm">
                    Selected Scene Objects
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="space-y-2">
                    {selectedObjects.map((obj) => (
                      <div
                        className="flex items-center gap-2 rounded px-2 py-1 text-sm hover:bg-muted/50"
                        key={obj.id}
                      >
                        <div className="text-muted-foreground">
                          {obj.type === "cube" && "⬜"}
                          {obj.type === "sphere" && "⚫"}
                          {obj.type === "plane" && "▭"}
                          {obj.type === "imported" && "📦"}
                        </div>
                        <div className="min-w-0 flex-1">
                          <div className="truncate" title={obj.name}>
                            {obj.name}
                          </div>
                        </div>
                        <div className="text-muted-foreground text-xs">
                          {obj.type}
                        </div>
                      </div>
                    ))}
                  </div>
                </CardContent>
              </Card>
            ) : hierarchy.nodes.length > 0 ? (
              <Card>
                <CardHeader>
                  <CardTitle className="text-sm">Model Hierarchy</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="space-y-1">
                    {hierarchy.rootNodes.map((nodeId) =>
                      renderHierarchyNode(nodeId)
                    )}
                  </div>
                </CardContent>
              </Card>
            ) : null}
          </div>
        </ScrollArea>
      </div>
    </div>
  );
}
