/**
 * DFF Model Viewer Component for Unified Sidebar
 * Based on RWMS DFF Viewer with Three.js WebGL rendering
 */

import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import React, { useCallback, useRef, useState } from "react";
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
import type { DffModel, ModelViewerState, Vector3 } from "~/types/dff";

// Simplified camera controller for sidebar use
class SidebarCameraController {
  readonly camera: THREE.PerspectiveCamera;
  private readonly controls: OrbitControls;

  constructor(camera: THREE.PerspectiveCamera, canvas: HTMLCanvasElement) {
    this.camera = camera;

    this.controls = new OrbitControls(camera, canvas);
    this.controls.enableDamping = true;
    this.controls.dampingFactor = 0.05;
    this.controls.enableZoom = true;
    this.controls.enableRotate = true;
    this.controls.enablePan = true;
  }

  update() {
    this.controls.update();
  }

  focusOnBounds(min: Vector3, max: Vector3) {
    const center = new THREE.Vector3(
      (min.x + max.x) / 2,
      (min.y + max.y) / 2,
      (min.z + max.z) / 2
    );
    const radius = Math.max(max.x - min.x, max.y - min.y, max.z - min.z);

    this.controls.target.copy(center);
    this.camera.position.set(
      center.x + radius * 1.5,
      center.y + radius * 1.5,
      center.z + radius * 1.5
    );
  }

  reset() {
    this.controls.target.set(0, 0, 0);
    this.camera.position.set(8, 8, 8);
  }
}

interface DffViewerProps {
  className?: string;
}

export function DffViewer({ className }: DffViewerProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const cameraControllerRef = useRef<SidebarCameraController | null>(null);
  const sceneRef = useRef<THREE.Scene | null>(null);
  const rendererRef = useRef<THREE.WebGLRenderer | null>(null);

  const [dffModel, setDffModel] = useState<DffModel | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [viewerState, setViewerState] = useState<ModelViewerState>({
    isLoading: false,
    showWireframe: false,
    showNormals: false,
    showAxes: true,
    backgroundColor: "#1e1e1e",
    camera: {
      position: { x: 8, y: 8, z: 8 },
      target: { x: 0, y: 0, z: 0 },
      distance: 8,
      azimuth: 35,
      elevation: 35,
    },
  });

  const { toast } = useToast();

  // Initialize Three.js scene
  const initializeScene = useCallback(() => {
    if (!canvasRef.current) return;

    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x1e_1e_1e);

    const camera = new THREE.PerspectiveCamera(
      75,
      canvasRef.current.clientWidth / canvasRef.current.clientHeight,
      0.1,
      1000
    );

    const renderer = new THREE.WebGLRenderer({
      canvas: canvasRef.current,
      antialias: true,
    });
    renderer.setSize(
      canvasRef.current.clientWidth,
      canvasRef.current.clientHeight
    );

    const controller = new SidebarCameraController(camera, canvasRef.current);
    controller.reset();

    // Add lights
    const ambientLight = new THREE.AmbientLight(0x40_40_40, 0.6);
    scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xff_ff_ff, 0.8);
    directionalLight.position.set(10, 10, 10);
    scene.add(directionalLight);

    // Add axes helper
    if (viewerState.showAxes) {
      const axesHelper = new THREE.AxesHelper(5);
      scene.add(axesHelper);
    }

    sceneRef.current = scene;
    rendererRef.current = renderer;
    cameraControllerRef.current = controller;

    // Start render loop
    const animate = () => {
      requestAnimationFrame(animate);
      controller.update();
      renderer.render(scene, camera);
    };
    animate();
  }, [viewerState.showAxes]);

  // Load model into scene
  const loadModelIntoScene = useCallback(
    (model: DffModel) => {
      if (!sceneRef.current) return;

      // Clear existing model
      sceneRef.current.children = sceneRef.current.children.filter(
        (child) =>
          !(child instanceof THREE.Group || child instanceof THREE.Mesh)
      );

      // Add axes helper back
      if (viewerState.showAxes) {
        const axesHelper = new THREE.AxesHelper(5);
        sceneRef.current.add(axesHelper);
      }

      // Create materials
      const defaultMaterial = new THREE.MeshPhongMaterial({
        color: 0xcc_cc_cc,
        wireframe: viewerState.showWireframe,
      });

      // Create geometries from model data
      model.geometries.forEach((geometry, index) => {
        try {
          const threeGeometry = new THREE.BufferGeometry();

          // Convert vertices - flatten Vector3 array to number array
          if (geometry.vertices && geometry.vertices.length > 0) {
            const positions = geometry.vertices.flatMap((v) => [v.x, v.y, v.z]);
            threeGeometry.setAttribute(
              "position",
              new THREE.Float32BufferAttribute(positions, 3)
            );
          }

          // Convert normals if available
          if (
            geometry.normals &&
            geometry.normals.length > 0 &&
            viewerState.showNormals
          ) {
            const normals = geometry.normals.flatMap((v) => [v.x, v.y, v.z]);
            threeGeometry.setAttribute(
              "normal",
              new THREE.Float32BufferAttribute(normals, 3)
            );
          }

          // Convert triangles to indices
          if (geometry.triangles && geometry.triangles.length > 0) {
            const indices = geometry.triangles.flatMap((t) => [t.a, t.b, t.c]);
            threeGeometry.setIndex(indices);
          }

          const mesh = new THREE.Mesh(threeGeometry, defaultMaterial);
          sceneRef.current?.add(mesh);
        } catch (err) {
          console.warn(`Failed to create geometry ${index}:`, err);
        }
      });

      // Calculate bounds and focus camera
      if (cameraControllerRef.current) {
        // Calculate bounds from all geometries
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
          if (geometry.vertices && geometry.vertices.length > 0) {
            for (const vertex of geometry.vertices) {
              min.x = Math.min(min.x, vertex.x);
              min.y = Math.min(min.y, vertex.y);
              min.z = Math.min(min.z, vertex.z);
              max.x = Math.max(max.x, vertex.x);
              max.y = Math.max(max.y, vertex.y);
              max.z = Math.max(max.z, vertex.z);
            }
          }
        }

        if (min.x !== Number.POSITIVE_INFINITY) {
          cameraControllerRef.current.focusOnBounds(
            { x: min.x, y: min.y, z: min.z },
            { x: max.x, y: max.y, z: max.z }
          );
        }
      }
    },
    [viewerState.showWireframe, viewerState.showNormals, viewerState.showAxes]
  );

  // Load DFF file
  const loadDffFile = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);

      const filePath = await open({
        filters: [
          {
            name: "DFF Model",
            extensions: ["dff"],
          },
        ],
      });

      if (!filePath) return;

      const model: DffModel = await invoke("parse_dff_model", {
        path: filePath,
      });

      setDffModel(model);

      // Initialize Three.js scene if canvas exists
      if (canvasRef.current && !sceneRef.current) {
        initializeScene();
      }

      // Load model into scene
      if (sceneRef.current) {
        loadModelIntoScene(model);
      }

      toast({
        title: "Model Loaded",
        description: `Successfully loaded DFF model with ${model.geometries.length} geometries`,
      });
    } catch (err) {
      console.error("Failed to load DFF file:", err);
      const errorMessage =
        err instanceof Error ? err.message : "Failed to load DFF file";
      setError(errorMessage);
      toast({
        title: "Load Failed",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setIsLoading(false);
    }
  }, [toast, initializeScene, loadModelIntoScene]);

  // Handle viewer state changes
  const handleStateChange = useCallback(
    (newState: Partial<ModelViewerState>) => {
      setViewerState((prev) => ({ ...prev, ...newState }));
    },
    []
  );

  // Update scene when state changes
  React.useEffect(() => {
    if (dffModel && sceneRef.current) {
      loadModelIntoScene(dffModel);
    }
  }, [dffModel, loadModelIntoScene]);

  return (
    <div className={cn("flex h-full flex-col", className)}>
      {/* Toolbar */}
      <div className="flex items-center gap-2 border-b p-2">
        <Button
          className="h-8"
          disabled={isLoading}
          onClick={loadDffFile}
          size="sm"
        >
          {isLoading ? "Loading..." : "Load DFF"}
        </Button>

        {dffModel && (
          <div className="ml-4 text-muted-foreground text-xs">
            {dffModel.geometries.length} geometries • {dffModel.frames.length}{" "}
            frames
          </div>
        )}
      </div>

      <div className="flex flex-1 overflow-hidden">
        {/* 3D Viewport */}
        <div className="flex-1">
          <canvas
            className="h-full w-full"
            ref={canvasRef}
            style={{ background: "#1e1e1e" }}
          />

          {error && (
            <div className="absolute inset-0 flex items-center justify-center bg-background/80 backdrop-blur-sm">
              <div className="text-center">
                <p className="mb-2 text-destructive">Error loading model</p>
                <p className="text-muted-foreground text-sm">{error}</p>
              </div>
            </div>
          )}
        </div>

        {/* Properties Panel */}
        <ScrollArea className="w-80 border-l">
          <div className="space-y-4 p-4">
            {/* Display Options */}
            <Card>
              <CardHeader>
                <CardTitle className="text-sm">Display</CardTitle>
              </CardHeader>
              <CardContent className="space-y-3">
                <div className="flex items-center space-x-2">
                  <Checkbox
                    checked={viewerState.showWireframe}
                    id="wireframe"
                    onCheckedChange={(checked) =>
                      handleStateChange({ showWireframe: checked as boolean })
                    }
                  />
                  <Label className="text-sm" htmlFor="wireframe">
                    Wireframe
                  </Label>
                </div>

                <div className="flex items-center space-x-2">
                  <Checkbox
                    checked={viewerState.showNormals}
                    id="normals"
                    onCheckedChange={(checked) =>
                      handleStateChange({ showNormals: checked as boolean })
                    }
                  />
                  <Label className="text-sm" htmlFor="normals">
                    Show Normals
                  </Label>
                </div>

                <div className="flex items-center space-x-2">
                  <Checkbox
                    checked={viewerState.showAxes}
                    id="axes"
                    onCheckedChange={(checked) =>
                      handleStateChange({ showAxes: checked as boolean })
                    }
                  />
                  <Label className="text-sm" htmlFor="axes">
                    Show Axes
                  </Label>
                </div>
              </CardContent>
            </Card>

            {/* Model Info */}
            {dffModel && (
              <Card>
                <CardHeader>
                  <CardTitle className="text-sm">Model Info</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="space-y-1 text-xs">
                    <div>RenderWare Version: {dffModel.rw_version}</div>
                    <div>Frames: {dffModel.frames.length}</div>
                    <div>Geometries: {dffModel.geometries.length}</div>
                    <div>Atomics: {dffModel.atomics.length}</div>
                  </div>
                </CardContent>
              </Card>
            )}

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
                      handleStateChange({
                        camera: { ...viewerState.camera, distance },
                      });
                      cameraControllerRef.current?.camera.position.setLength(
                        distance
                      );
                    }}
                    type="number"
                    value={viewerState.camera.distance}
                  />
                </div>
              </CardContent>
            </Card>
          </div>
        </ScrollArea>
      </div>
    </div>
  );
}
