import { Html } from "@react-three/drei";
import { useThree } from "@react-three/fiber";
import { useCallback, useEffect, useRef, useState } from "react";
import { Camera, Spherical, Vector3 } from "three";
import { useEditorStore } from "~/store/editor-store";

interface ViewportControlsProps {
  children: React.ReactNode;
}

export default function ViewportControls({ children }: ViewportControlsProps) {
  const { camera, gl } = useThree();
  const { selectedObjectId, mapData, setCameraPosition, setCameraTarget, tool } = useEditorStore();

  const [isDragging, setIsDragging] = useState(false);
  const [dragType, setDragType] = useState<"rotate" | "pan" | "zoom" | "orbit" | null>(null);
  const [lastMousePos, setLastMousePos] = useState({ x: 0, y: 0 });
  const [isRightMouseDown, setIsRightMouseDown] = useState(false);
  const [isLeftMouseDown, setIsLeftMouseDown] = useState(false);
  const [isMiddleMouseDown, setIsMiddleMouseDown] = useState(false);
  const [isAltDown, setIsAltDown] = useState(false);
  const [cameraSpeed, setCameraSpeed] = useState(1);
  const [isGameMode, setIsGameMode] = useState(false);

  const cameraRef = useRef<Camera>(camera);
  const orbitTargetRef = useRef<Vector3>(new Vector3(0, 0, 0));

  // Update camera ref when camera changes
  useEffect(() => {
    cameraRef.current = camera;
  }, [camera]);

  // Keyboard event handlers
  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      const key = event.key.toLowerCase();

      // Prevent default for our shortcuts
      if (
        [
          "w",
          "a",
          "s",
          "d",
          "q",
          "e",
          "f",
          "g",
          "r",
          "t",
          "space",
          "alt",
          "ctrl",
          "shift",
        ].includes(key)
      ) {
        event.preventDefault();
      }

      // WASD movement (only when right mouse is held)
      if (isRightMouseDown) {
        const moveSpeed = cameraSpeed * 0.1;
        const forward = new Vector3();
        const right = new Vector3();
        const up = new Vector3(0, 1, 0);

        camera.getWorldDirection(forward);
        right.crossVectors(forward, up).normalize();

        switch (key) {
          case "w":
            camera.position.add(forward.multiplyScalar(moveSpeed));
            break;
          case "s":
            camera.position.add(forward.multiplyScalar(-moveSpeed));
            break;
          case "a":
            camera.position.add(right.multiplyScalar(-moveSpeed));
            break;
          case "d":
            camera.position.add(right.multiplyScalar(moveSpeed));
            break;
          case "q":
            camera.position.add(up.multiplyScalar(-moveSpeed));
            break;
          case "e":
            camera.position.add(up.multiplyScalar(moveSpeed));
            break;
        }
      }

      // Tool shortcuts
      switch (key) {
        case "f":
          focusOnSelectedObject();
          break;
        case "g":
          setIsGameMode(!isGameMode);
          break;
        case " ":
          event.preventDefault();
          // Cycle through tools (implement based on your tool system)
          break;
      }

      // Modifier keys
      if (key === "alt") setIsAltDown(true);
    },
    [isRightMouseDown, cameraSpeed, isGameMode],
  );

  const handleKeyUp = useCallback((event: KeyboardEvent) => {
    const key = event.key.toLowerCase();
    if (key === "alt") setIsAltDown(false);
  }, []);

  // Focus on selected object (F key)
  const focusOnSelectedObject = useCallback(() => {
    if (selectedObjectId && mapData) {
      const selectedObject = mapData.objects.find((obj) => obj.id === selectedObjectId);
      if (selectedObject) {
        const target = new Vector3(
          selectedObject.position.x,
          selectedObject.position.y,
          selectedObject.position.z,
        );

        // Move camera to look at the object
        const distance = 5;
        const direction = new Vector3().subVectors(camera.position, target).normalize();
        const newPosition = target.clone().add(direction.multiplyScalar(distance));

        camera.position.copy(newPosition);
        camera.lookAt(target);
        orbitTargetRef.current.copy(target);

        setCameraPosition({ x: newPosition.x, y: newPosition.y, z: newPosition.z });
        setCameraTarget({ x: target.x, y: target.y, z: target.z });
      }
    }
  }, [selectedObjectId, mapData, camera, setCameraPosition, setCameraTarget]);

  // Mouse event handlers
  const handleMouseDown = useCallback(
    (event: MouseEvent) => {
      // Don't handle mouse events when Shift is pressed (let marquee selection handle it)
      if (event.shiftKey) return;

      // Don't handle mouse events when in add mode (let canvas handle object placement)
      if (tool === "add") return;

      const button = event.button;
      const mousePos = { x: event.clientX, y: event.clientY };

      setLastMousePos(mousePos);
      setIsDragging(true);

      if (button === 0) {
        // Left mouse button
        setIsLeftMouseDown(true);
        if (isAltDown) {
          setDragType("orbit");
        } else {
          setDragType("rotate");
        }
      } else if (button === 1) {
        // Middle mouse button
        setIsMiddleMouseDown(true);
        if (isAltDown) {
          setDragType("pan");
        } else {
          setDragType("pan");
        }
      } else if (button === 2) {
        // Right mouse button
        setIsRightMouseDown(true);
        setDragType("rotate");
      }
    },
    [isAltDown, tool],
  );

  const handleMouseUp = useCallback(() => {
    setIsDragging(false);
    setDragType(null);
    setIsLeftMouseDown(false);
    setIsMiddleMouseDown(false);
    setIsRightMouseDown(false);
  }, []);

  const handleMouseMove = useCallback(
    (event: MouseEvent) => {
      // Don't handle mouse events when Shift is pressed (let marquee selection handle it)
      if (event.shiftKey) return;

      // Don't handle mouse events when in add mode (let canvas handle object placement)
      if (tool === "add") return;

      if (!isDragging || !dragType) return;

      const mousePos = { x: event.clientX, y: event.clientY };
      const deltaX = mousePos.x - lastMousePos.x;
      const deltaY = mousePos.y - lastMousePos.y;

      const sensitivity = 0.002;

      switch (dragType) {
        case "rotate":
          if (isRightMouseDown || (isLeftMouseDown && !isAltDown)) {
            // Rotate camera around target
            const spherical = new Spherical();
            spherical.setFromVector3(camera.position.clone().sub(orbitTargetRef.current));

            spherical.theta -= deltaX * sensitivity;
            spherical.phi += deltaY * sensitivity;
            spherical.phi = Math.max(0.1, Math.min(Math.PI - 0.1, spherical.phi));

            const newPosition = new Vector3();
            newPosition.setFromSpherical(spherical);
            newPosition.add(orbitTargetRef.current);

            camera.position.copy(newPosition);
            camera.lookAt(orbitTargetRef.current);
          }
          break;

        case "pan":
          if (isMiddleMouseDown || (isLeftMouseDown && isRightMouseDown)) {
            // Pan camera
            const panSpeed = 0.01;
            const panVector = new Vector3(-deltaX * panSpeed, deltaY * panSpeed, 0);

            camera.position.add(panVector);
            orbitTargetRef.current.add(panVector);
          }
          break;

        case "orbit":
          if (isLeftMouseDown && isAltDown) {
            // Orbit around target
            const spherical = new Spherical();
            spherical.setFromVector3(camera.position.clone().sub(orbitTargetRef.current));

            spherical.theta -= deltaX * sensitivity;
            spherical.phi += deltaY * sensitivity;
            spherical.phi = Math.max(0.1, Math.min(Math.PI - 0.1, spherical.phi));

            const newPosition = new Vector3();
            newPosition.setFromSpherical(spherical);
            newPosition.add(orbitTargetRef.current);

            camera.position.copy(newPosition);
            camera.lookAt(orbitTargetRef.current);
          }
          break;
      }

      setLastMousePos(mousePos);
    },
    [
      isDragging,
      dragType,
      lastMousePos,
      isRightMouseDown,
      isLeftMouseDown,
      isMiddleMouseDown,
      isAltDown,
      camera,
      tool,
    ],
  );

  const handleWheel = useCallback(
    (event: WheelEvent) => {
      event.preventDefault();

      const zoomSpeed = 0.1;
      const zoomDirection = event.deltaY > 0 ? 1 : -1;

      if (isRightMouseDown) {
        // Adjust camera speed
        setCameraSpeed((prev) => Math.max(0.1, Math.min(10, prev + zoomDirection * 0.1)));
      } else {
        // Zoom camera
        const zoomFactor = 1 + zoomDirection * zoomSpeed;
        camera.position.multiplyScalar(zoomFactor);

        setCameraPosition({
          x: camera.position.x,
          y: camera.position.y,
          z: camera.position.z,
        });
      }
    },
    [isRightMouseDown, camera, setCameraPosition],
  );

  // Fullscreen toggle
  const toggleFullscreen = useCallback(() => {
    if (!document.fullscreenElement) {
      gl.domElement.requestFullscreen();
    } else {
      document.exitFullscreen();
    }
  }, [gl.domElement]);

  // Event listeners
  useEffect(() => {
    const canvas = gl.domElement;

    canvas.addEventListener("mousedown", handleMouseDown);
    canvas.addEventListener("mouseup", handleMouseUp);
    canvas.addEventListener("mousemove", handleMouseMove);
    canvas.addEventListener("wheel", handleWheel);
    canvas.addEventListener("contextmenu", () => {});

    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);

    // F11 for fullscreen
    const handleF11 = (event: KeyboardEvent) => {
      if (event.key === "F11") {
        event.preventDefault();
        toggleFullscreen();
      }
    };

    window.addEventListener("keydown", handleF11);

    return () => {
      canvas.removeEventListener("mousedown", handleMouseDown);
      canvas.removeEventListener("mouseup", handleMouseUp);
      canvas.removeEventListener("mousemove", handleMouseMove);
      canvas.removeEventListener("wheel", handleWheel);
      canvas.removeEventListener("contextmenu", () => {});

      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
      window.removeEventListener("keydown", handleF11);
    };
  }, [
    handleMouseDown,
    handleMouseUp,
    handleMouseMove,
    handleWheel,
    handleKeyDown,
    handleKeyUp,
    toggleFullscreen,
    gl.domElement,
  ]);

  // Update camera state in store
  useEffect(() => {
    setCameraPosition({
      x: camera.position.x,
      y: camera.position.y,
      z: camera.position.z,
    });
    setCameraTarget({
      x: orbitTargetRef.current.x,
      y: orbitTargetRef.current.y,
      z: orbitTargetRef.current.z,
    });
  }, [camera.position, setCameraPosition, setCameraTarget]);

  return (
    <>
      {children}
      <Html>
        {/* Game mode overlay */}
        {isGameMode && (
          <div className="absolute top-4 left-4 bg-black bg-opacity-50 text-white p-2 rounded">
            GAME MODE
          </div>
        )}
        {/* Camera speed indicator */}
        {isRightMouseDown && (
          <div className="absolute bottom-4 right-4 bg-black bg-opacity-50 text-white p-2 rounded">
            Speed: {cameraSpeed.toFixed(1)}x
          </div>
        )}
      </Html>
    </>
  );
}
