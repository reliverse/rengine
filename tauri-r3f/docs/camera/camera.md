# Camera System - Implementation Plan

## Overview

This document outlines the implementation plan for Rengine's camera system. The system will use **quaternions as the single source of truth** for rotation, ensuring stable interpolation, avoiding gimbal lock, and providing a foundation for future features like animations and cinematic cameras. The system will integrate seamlessly with React Three Drei's `OrbitControls` for intuitive user interaction, provide professional fly camera controls for FPS-style navigation, and maintain precise control through manual inputs.

**Status**: 🚧 **Not Yet Implemented** - This is an implementation plan.

**Architecture**: React Three Fiber + React Three Drei + Zustand Store + Tauri

## Architecture

### Core Principle

**Quaternions are canonical** - All camera rotation will be stored and manipulated as normalized quaternions `[x, y, z, w]`. Euler angles will be used **only** for UI display and user input, never stored as state.

### State Management

Camera state will be managed in the scene store (`src/stores/scene-store.ts`). The following additions are needed:

```typescript
// Add to SceneState interface
interface SceneState {
  // ... existing fields ...
  
  // Camera state (TO BE ADDED)
  cameraPosition: [number, number, number];
  cameraTarget: [number, number, number];
  cameraQuaternion: [number, number, number, number]; // Normalized quaternion [x, y, z, w]
  cameraBookmarks: CameraBookmark[];
  cameraHistory: CameraState[]; // For undo/redo (max 50 states)
  cameraHistoryIndex: number; // Current position in history
  
  // Camera settings
  cameraSpeed: number; // Base movement speed for fly mode (default: 5.0)
  cameraRotationSpeed: number; // Mouse sensitivity for rotation (default: 0.002)
  cameraZoomSpeed: number; // Zoom speed multiplier (default: 1.0)
  cameraPanSpeed: number; // Pan speed multiplier (default: 1.0)
  cameraFocusDistance: number; // Distance multiplier for focus operations (default: 1.5)
  
  // Camera configuration
  cameraFov: number; // Field of view in degrees (default: 50)
  cameraNear: number; // Near clipping plane (default: 0.1)
  cameraFar: number; // Far clipping plane (default: 1000)
  cameraSmoothingMode: CameraSmoothingMode; // Interpolation mode (default: "ease-in-out")
  cameraSmoothingDuration: number; // Transition duration in seconds (default: 0.3)
  cameraCollisionEnabled: boolean; // Enable collision detection (default: false)
  cameraCollisionRadius: number; // Collision sphere radius (default: 0.5)
  cameraAutoPivot: boolean; // Auto-update pivot on selection (default: true)
  cameraConstraints: CameraConstraints | null; // Position/rotation limits (default: null)
  
  // Viewport settings
  viewportOverlays: {
    grid: boolean; // Show grid overlay (default: true)
    guides: boolean; // Show guide lines (default: false)
    safeArea: boolean; // Show safe area guides (default: false)
    cameraGizmo: boolean; // Show camera gizmo (default: true)
    frustumCulling: boolean; // Enable frustum culling visualization (default: false)
  };
  
  // Multi-viewport support (future)
  activeViewport: number; // Active viewport index (default: 0)
  viewports: ViewportConfig[]; // Multiple viewport configurations
}

// Add to SceneActions interface
interface SceneActions {
  // ... existing actions ...
  
  // Camera actions (TO BE ADDED)
  setCameraPosition(position: [number, number, number], addToHistory?: boolean): void;
  setCameraTarget(target: [number, number, number], addToHistory?: boolean): void;
  setCameraQuaternion(quaternion: [number, number, number, number], addToHistory?: boolean): void;
  addCameraBookmark(name: string): void;
  deleteCameraBookmark(id: string): void;
  renameCameraBookmark(id: string, name: string): void;
  loadCameraBookmark(id: string, smooth?: boolean): void;
  
  // Camera history (undo/redo)
  pushCameraState(): void; // Save current camera state to history
  undoCameraState(): void; // Restore previous camera state
  redoCameraState(): void; // Restore next camera state
  clearCameraHistory(): void; // Clear camera history
  
  // Camera control actions
  focusOnSelection(options?: FocusOptions): void; // Focus camera on selected object(s) - F key
  setCameraPreset(preset: CameraPreset, smooth?: boolean): void; // Set camera to preset view
  setCameraSpeed(speed: number): void; // Set fly mode movement speed
  setCameraRotationSpeed(speed: number): void; // Set mouse sensitivity
  setCameraZoomSpeed(speed: number): void; // Set zoom speed
  setCameraPanSpeed(speed: number): void; // Set pan speed
  setCameraFocusDistance(distance: number): void; // Set focus distance multiplier
  
  // Camera configuration actions
  setCameraFov(fov: number): void; // Set field of view
  setCameraSmoothingMode(mode: CameraSmoothingMode): void; // Set interpolation mode
  setCameraSmoothingDuration(duration: number): void; // Set transition duration
  setCameraCollisionEnabled(enabled: boolean): void; // Enable/disable collision
  setCameraCollisionRadius(radius: number): void; // Set collision radius
  setCameraAutoPivot(enabled: boolean): void; // Enable/disable auto-pivot
  setCameraConstraints(constraints: CameraConstraints | null): void; // Set position/rotation limits
  toggleViewportOverlay(overlay: keyof ViewportOverlays): void; // Toggle overlay visibility
}

// Camera Bookmark Structure (TO BE ADDED)
interface CameraBookmark {
  id: string;
  name: string;
  position: [number, number, number];
  quaternion: [number, number, number, number]; // Normalized quaternion
  target?: [number, number, number]; // Optional for OrbitControls compatibility
}

// Camera Preset Types
type CameraPreset = "front" | "back" | "left" | "right" | "top" | "bottom" | "perspective";

// Camera Smoothing Modes
type CameraSmoothingMode = 
  | "linear"           // Linear interpolation
  | "ease-in-out"      // Smooth acceleration/deceleration (default)
  | "ease-in"          // Slow start, fast end
  | "ease-out"         // Fast start, slow end
  | "exponential"      // Exponential decay
  | "spring"           // Spring physics simulation
  | "instant";         // No smoothing (immediate)

// Camera Constraints
interface CameraConstraints {
  positionBounds?: {
    min: [number, number, number];
    max: [number, number, number];
  };
  rotationLimits?: {
    minPitch: number; // In degrees
    maxPitch: number;
    minYaw: number;
    maxYaw: number;
  };
  distanceLimits?: {
    min: number;
    max: number;
  };
}

// Focus Options
interface FocusOptions {
  padding?: number; // Padding around objects (default: 0.1)
  aspectRatio?: number; // Viewport aspect ratio (default: auto)
  smooth?: boolean; // Smooth transition (default: true)
  duration?: number; // Transition duration in seconds (default: 0.5)
  maintainRotation?: boolean; // Keep current rotation (default: false)
  focusOnCenter?: boolean; // Focus on center instead of bounds (default: false)
}

// Camera State (for history)
interface CameraState {
  position: [number, number, number];
  quaternion: [number, number, number, number];
  target: [number, number, number];
  fov: number;
  timestamp: number;
}

// Viewport Configuration (multi-viewport support)
interface ViewportConfig {
  id: string;
  name: string;
  cameraState: CameraState;
  viewType: CameraPreset | "perspective";
  locked: boolean; // Lock camera movement
  visible: boolean;
}
```

### Default Values

```typescript
// Add to src/lib/defaults.ts
export const CAMERA_DEFAULTS = {
  POSITION: [5, 5, 5] as [number, number, number],
  TARGET: [0, 0, 0] as [number, number, number],
  QUATERNION: [0, 0, 0, 1] as [number, number, number, number], // Identity quaternion
  
  // Camera settings
  SPEED: 5.0,
  ROTATION_SPEED: 0.002,
  ZOOM_SPEED: 1.0,
  PAN_SPEED: 1.0,
  FOCUS_DISTANCE: 1.5,
  
  // Camera properties
  FOV: 50,
  NEAR: 0.1,
  FAR: 1000,
  
  // Smoothing
  SMOOTHING_MODE: "ease-in-out" as CameraSmoothingMode,
  SMOOTHING_DURATION: 0.3,
  
  // Collision
  COLLISION_ENABLED: false,
  COLLISION_RADIUS: 0.5,
  
  // Orbit controls
  ORBIT_DAMPING: true,
  ORBIT_DAMPING_FACTOR: 0.05,
  ORBIT_MIN_DISTANCE: 0.1,
  ORBIT_MAX_DISTANCE: 1000,
  ORBIT_MIN_POLAR_ANGLE: 0, // radians
  ORBIT_MAX_POLAR_ANGLE: Math.PI, // radians
  
  // Auto-pivot
  AUTO_PIVOT: true,
  
  // History
  MAX_HISTORY_STATES: 50,
  
  // Focus
  FOCUS_PADDING: 0.1,
  FOCUS_DURATION: 0.5,
  
  // Viewport overlays
  OVERLAY_GRID: true,
  OVERLAY_GUIDES: false,
  OVERLAY_SAFE_AREA: false,
  OVERLAY_CAMERA_GIZMO: true,
  OVERLAY_FRUSTUM_CULLING: false,
};
```

## Camera Controls

### OrbitControls Integration

**Implementation**: Use React Three Drei's `<OrbitControls>` component in `SceneRenderer` or a dedicated camera component.

The camera will use React Three Drei's `OrbitControls` for interactive manipulation, providing professional viewport controls:

#### Perspective View Controls

**Looking Around:**
- **Hold LMB + Drag**: Moves camera forward/backward and rotates left/right
- **Hold RMB + Drag**: Rotates camera to look around the level
- **Hold LMB + Hold RMB + Drag** OR **Hold MMB + Drag**: Moves camera up/down/left/right without rotation (pan)
- **Scroll MMB**: Moves camera forward/backward by increments (zoom)

**Orbiting Around Object/Pivot:**
- **Press F**: Focus camera on selected object
- **Alt + Hold LMB + Drag**: Orbits camera around pivot/point of interest
- **Alt + Hold RMB + Drag**: Dollies (zooms) camera toward/away from pivot
- **Alt + Hold MMB + Drag**: Tracks camera left/right/up/down

**Behavior**:
- Smooth damping for all camera movements
- Camera maintains focus on target point during orbit
- Zoom limits prevent camera from going too close/far
- Configurable pan speed for optimal feel
- Alt modifier enables orbit/dolly/track modes (Maya/Unity-style)

**Implementation Details**:
- When OrbitControls change, extract the camera's quaternion from the THREE.Camera object and store in Zustand state
- Use `useFrame` or OrbitControls `onChange` callback to sync state
- React Three Drei's OrbitControls automatically handles camera updates via R3F's imperative API

**Example Implementation**:
```typescript
// In SceneRenderer or dedicated camera component
import { OrbitControls } from "@react-three/drei";
import { useFrame, useThree } from "@react-three/fiber";
import { useEffect, useRef } from "react";
import { useSceneStore, CAMERA_DEFAULTS } from "~/stores/scene-store";

function CameraControls() {
  const { camera } = useThree();
  const controlsRef = useRef<any>(null);
  const { 
    setCameraPosition, 
    setCameraQuaternion, 
    setCameraTarget,
    cameraZoomSpeed,
    cameraPanSpeed,
  } = useSceneStore();
  
  // Sync OrbitControls changes to store
  useFrame(() => {
    if (controlsRef.current) {
      const q = camera.quaternion;
      setCameraQuaternion([q.x, q.y, q.z, q.w]);
      setCameraPosition([camera.position.x, camera.position.y, camera.position.z]);
      setCameraTarget([
        controlsRef.current.target.x,
        controlsRef.current.target.y,
        controlsRef.current.target.z,
      ]);
    }
  });
  
  // Handle F key for focus on selection
  useEffect(() => {
    const handleKeyPress = (e: KeyboardEvent) => {
      // Check if not typing in input field
      const target = e.target as HTMLElement;
      if (target.tagName === "INPUT" || target.tagName === "TEXTAREA") {
        return;
      }
      
      // F key: Focus on selection
      if (e.key === "f" || e.key === "F") {
        e.preventDefault();
        useSceneStore.getState().focusOnSelection();
      }
      
      // Alt + LMB/RMB/MMB for orbit/dolly/track (handled in mouse events)
      // This is handled separately in the OrbitControls component
    };
    
    window.addEventListener("keydown", handleKeyPress);
    return () => window.removeEventListener("keydown", handleKeyPress);
  }, []);
  
  return (
    <OrbitControls
      ref={controlsRef}
      enableDamping={CAMERA_DEFAULTS.ORBIT_DAMPING}
      dampingFactor={CAMERA_DEFAULTS.ORBIT_DAMPING_FACTOR}
      enableZoom
      enableRotate
      enablePan
      minDistance={CAMERA_DEFAULTS.ORBIT_MIN_DISTANCE}
      maxDistance={CAMERA_DEFAULTS.ORBIT_MAX_DISTANCE}
      minPolarAngle={CAMERA_DEFAULTS.ORBIT_MIN_POLAR_ANGLE}
      maxPolarAngle={CAMERA_DEFAULTS.ORBIT_MAX_POLAR_ANGLE}
      zoomSpeed={cameraZoomSpeed}
      panSpeed={cameraPanSpeed}
      // Alt modifier enables orbit/dolly/track modes
      // Note: This requires custom handling as drei OrbitControls doesn't support Alt modifier by default
      // Will need custom implementation or drei OrbitControls extension
    />
  );
}
```

**Alt Modifier Implementation**:
Since React Three Drei's OrbitControls doesn't natively support Alt modifier for orbit/dolly/track modes, we'll need to implement custom handlers:

```typescript
// Custom Alt modifier handling for orbit/dolly/track
useEffect(() => {
  const handleMouseDown = (e: MouseEvent) => {
    if (e.altKey) {
      // Alt + LMB: Orbit mode
      // Alt + RMB: Dolly mode  
      // Alt + MMB: Track mode
      // Disable default OrbitControls and handle manually
    }
  };
  
  // Add event listeners for Alt modifier modes
  // Implementation details in fly-camera-controls.tsx
}, []);
```

### Fly Camera Controls

**Implementation**: Create `src/hooks/use-fly-camera-controls.ts` hook and `src/components/scene/fly-camera-controls.tsx` component.

Rengine provides professional editor-style fly camera controls for intuitive viewport navigation:

#### Activation

- **Right Mouse Button (Hold)**: Activates fly mode
- **Right Mouse Button (Release)**: Exits fly mode and returns control to OrbitControls
- OrbitControls are automatically disabled while fly mode is active

#### Mouse Look (Rotation)

While Right Mouse Button is held, mouse movement rotates the camera:

- **Mouse Horizontal**: Yaw rotation (look left/right)
- **Mouse Vertical**: Pitch rotation (look up/down)
- Rotation is applied via quaternion updates (maintains quaternion-based architecture)
- Pitch is clamped to ±89° to prevent camera flipping
- Works without pointer lock - simply hold RMB and drag

#### Keyboard Movement

While Right Mouse Button is held, keyboard controls move the camera:

**Movement Controls:**
- **W** / **Numpad8** / **Up Arrow**: Move camera forward
- **S** / **Numpad2** / **Down Arrow**: Move camera backward
- **A** / **Numpad4** / **Left Arrow**: Move camera left
- **D** / **Numpad6** / **Right Arrow**: Move camera right

**Vertical Movement (Global Space):**
- **E** / **Numpad9** / **Page Up**: Move camera up in Global space
- **Q** / **Numpad7** / **Page Down**: Move camera down in Global space

**Vertical Movement (Local Space):**
- **R**: Move camera up in Local space
- **F**: Move camera down in Local space

**Camera Settings:**
- **Hold RMB + Scroll Wheel**: Increase/decrease camera movement speed
- **Z** / **Numpad1**: Raise Field of View
- **C** / **Numpad3**: Lower Field of View

**Important**:
- A and D keys move left/right (strafe), they do NOT rotate the camera
- All rotation is controlled exclusively by mouse movement while RMB is held
- E/Q use Global space (world up/down), R/F use Local space (camera up/down)
- Movement speed can be adjusted with scroll wheel while holding RMB

Movement characteristics:

- **Frame-rate independent**: Uses delta time for consistent speed
- **Local space**: Forward/backward/left/right movement relative to camera orientation
- **Global space**: Up/down movement (E/Q) uses world coordinates
- **Local space**: Up/down movement (R/F) uses camera-relative coordinates
- **Speed adjustment**: Scroll wheel while holding RMB adjusts movement speed dynamically
- **FOV adjustment**: Z/C keys adjust camera field of view in real-time

#### Default Settings

```typescript
{
  moveSpeed: 5.0,         // units per second
  mouseSensitivity: 0.002, // radians per pixel
  scrollSpeedMultiplier: 0.1, // Speed adjustment per scroll tick
  minMoveSpeed: 0.1,     // Minimum movement speed
  maxMoveSpeed: 100.0,   // Maximum movement speed
  fovAdjustSpeed: 5.0,   // FOV change per keypress in degrees
  minFov: 10.0,          // Minimum FOV in degrees
  maxFov: 170.0,         // Maximum FOV in degrees
  usePointerLock: false,   // No pointer lock
  pitchClamp: Math.PI / 2 - 0.01 // ±89° pitch limit (prevents gimbal lock)
}
```

**Behavior**:
- Smooth, responsive movement speed
- Configurable mouse sensitivity for optimal feel
- Scroll wheel adjusts speed dynamically (no Shift/Alt modifiers needed)
- Real-time FOV adjustment with Z/C keys
- Clear distinction between Global and Local space movement

#### Integration Details

**To be implemented**:
- Updates `cameraPosition` and `cameraQuaternion` in scene store every frame during fly mode using `useFrame`
- Does not modify `cameraTarget` during fly mode
- On exit, automatically updates `cameraTarget` to be 5 units in front of camera for OrbitControls compatibility
- Fully compatible with Views tab and camera bookmarks
- Context menu should be disabled on canvas to allow RMB for camera control (check `src/components/scene-canvas.tsx`)
- Ignores input when:
  - Typing in text fields or textareas (check `event.target.tagName`)
  - Clicking on UI elements (buttons, dialogs, menus)
  - Interacting with sidebar panels

**Implementation Notes**:
- Use `useThree()` to access camera and controls
- Use `useFrame()` for frame-rate independent updates
- Disable OrbitControls during fly mode: `controls.enabled = false`
- Use `document.addEventListener` for global keyboard/mouse events
- Check if input is focused on editable elements before processing
- **Key Support**:
  - Support numpad keys (Numpad1-9) as alternatives to main keys
  - Support arrow keys (Up/Down/Left/Right) as alternatives to WASD
  - Support Page Up/Page Down as alternatives to E/Q
  - Handle scroll wheel for speed adjustment while RMB is held
  - Implement FOV adjustment with Z/C keys
  - Distinguish between Global space (E/Q) and Local space (R/F) for vertical movement

### Manual Input Controls

**Implementation**: Create `src/components/views-tab.tsx` component (or integrate into existing sidebar).

The **Views** tab provides precise manual control for camera positioning:

- **Position Inputs**: Direct X, Y, Z coordinate editing (number inputs)
- **Rotation Inputs**: Euler angles (RX, RY, RZ) in degrees for user-friendly input
  - Inputs are converted to quaternions immediately using `camera-utils.ts`
  - Display values are derived from the stored quaternion for display
- **Camera Speed Settings**: Adjustable movement and rotation speeds
- **Camera Preset Buttons**: Quick buttons for Front, Side, Top, etc. views
- **Focus Button**: Button to focus on selection (alternative to F key)

**UI Integration**:
- Add a "Views" tab to the sidebar (check existing sidebar structure)
- Use shadcn/ui components (`Input`, `Label`, `Button`, `Slider`) for consistency
- Real-time updates: onChange handlers call store actions immediately
- Camera preset buttons arranged in grid for quick access

### Bidirectional Synchronization

**Implementation**: Create sync components/hooks to maintain bidirectional sync between OrbitControls, fly camera controls, and manual inputs.

The system will maintain bidirectional sync:

1. **OrbitControls → Store**: When user manipulates camera via controls, quaternion is extracted and stored
   - Use OrbitControls `onChange` callback or `useFrame` to detect changes
   - Extract quaternion: `camera.quaternion.toArray()` and normalize
   - Call `setCameraQuaternion()` and `setCameraPosition()`

2. **Fly Camera → Store**: When user navigates in fly mode, position and quaternion are updated continuously
   - Update store every frame in `useFrame` hook
   - Direct camera manipulation, then sync to store

3. **Store → OrbitControls**: When user edits inputs or exits fly mode, quaternion is applied to camera and OrbitControls are updated
   - Create `CameraQuaternionSync` component that watches store changes
   - Apply quaternion to camera: `camera.quaternion.fromArray(quaternion)`
   - Update OrbitControls target if needed: `controls.target.set(...)`
   - Skip sync during fly mode to prevent conflicts

### Control Mode Switching

The camera system seamlessly switches between control modes:

- **OrbitControls Mode**: Default mode for scene navigation
- **Fly Mode**: Activated by holding Right Mouse Button, provides FPS-style navigation
- **Manual Input Mode**: Via Views tab, provides precise coordinate editing

**Behavior**:
- Mode switching preserves camera state (position, rotation, target)
- Smooth transitions between modes (no jarring camera jumps)
- Visual feedback when entering/exiting fly mode (optional UI indicator)
- Keyboard shortcuts work consistently across modes (F for focus, number keys for bookmarks)

## Views Tab

**Implementation**: Create `src/components/views-tab.tsx` component.

The Views tab will provide comprehensive camera management:

### Camera Position Controls

Three number inputs for X, Y, Z coordinates. Changes are applied immediately to the camera via `setCameraPosition()`.

**Implementation**:
```typescript
// Use shadcn Input components
<Input type="number" value={cameraPosition[0]} onChange={...} />
```

### Camera Rotation Controls

Three number inputs for rotation angles:
- **RX (Pitch)**: Rotation around X-axis in degrees
- **RY (Yaw)**: Rotation around Y-axis in degrees  
- **RZ (Roll)**: Rotation around Z-axis in degrees

**Important**: These values are derived from the stored quaternion for display using `eulerDegreesFromQuaternion()`. When edited, they are converted to quaternion using `quaternionFromEulerDegrees()` and stored.

### Camera Bookmarks

Save and restore camera positions (camera bookmarks):

- **Add Bookmark**: Save current camera position, rotation, and target
- **Load Bookmark**: Restore saved camera state (with smooth transition option)
- **Rename Bookmark**: Update bookmark name
- **Delete Bookmark**: Remove saved camera position
- **Quick Access**: Keyboard shortcuts for bookmarks (1-9 keys for quick bookmark access)

**Persistence**: Bookmarks are stored in scene state and will be saved/loaded with scene files via `src/utils/scene-persistence.ts`. Update `SceneFileData` interface to include `cameraBookmarks`.

**Features**:
- Bookmarks can be assigned to number keys (1-9) for quick access
- Loading bookmark can optionally animate camera transition
- Bookmarks show preview thumbnail (future enhancement)

## Camera Utilities API

**Implementation**: Create `src/utils/camera-utils.ts` with pure functions for quaternion operations.

The camera utilities will provide pure functions for quaternion operations:

### Conversion Functions

#### `quaternionFromEulerDegrees(euler: [number, number, number]): [number, number, number, number]`

Converts Euler angles (in degrees) to a normalized quaternion.

```typescript
const quaternion = quaternionFromEulerDegrees([45, 30, 0]);
// Returns normalized quaternion [x, y, z, w]
```

#### `eulerDegreesFromQuaternion(quaternion: [number, number, number, number]): [number, number, number]`

Converts quaternion to Euler angles (in degrees) for UI display.

```typescript
const euler = eulerDegreesFromQuaternion([0, 0, 0, 1]);
// Returns [rx, ry, rz] in degrees
```

### Camera Operations

#### `getCameraQuaternion(camera: THREE.Camera): [number, number, number, number]`

Extracts normalized quaternion from camera object.

```typescript
const quaternion = getCameraQuaternion(camera);
// Returns normalized quaternion [x, y, z, w]
```

#### `applyQuaternionToCamera(camera: THREE.Camera, quaternion: [number, number, number, number], currentTarget?: Vector3 | [number, number, number], distance?: number): Vector3 | null`

Applies quaternion to camera and optionally updates position based on target and distance.

```typescript
const target = applyQuaternionToCamera(
  camera,
  [0, 0, 0, 1], // Identity quaternion
  [0, 0, 0],    // Target point
  10            // Distance from target
);
```

#### `quaternionFromCameraLookAt(camera: THREE.Camera, target: Vector3 | [number, number, number]): [number, number, number, number]`

Derives quaternion from camera position and look-at target.

```typescript
const quaternion = quaternionFromCameraLookAt(
  camera,
  [0, 0, 0] // Target point
);
```

### Camera Utility Functions

#### `focusOnSelection(objects: SceneObject[], camera: THREE.Camera, controls: OrbitControls, focusDistance?: number): void`

Focuses camera on selected objects (F key behavior). Calculates bounding box of all objects and frames them in view.

```typescript
focusOnSelection(
  selectedObjects,
  camera,
  controls,
  1.5 // Optional: distance multiplier (default from CAMERA_DEFAULTS.FOCUS_DISTANCE)
);
```

**UE5 Behavior** (Enhanced AAA-ready):
- Calculates combined bounding box of all selected objects
- Handles empty selection by focusing on all scene objects
- Supports padding around objects for better framing
- Respects viewport aspect ratio for optimal framing
- Positions camera at appropriate distance to frame all objects
- Smoothly animates camera to new position (optional, can be instant)
- Centers target on bounding box center (or object center if `focusOnCenter` is true)
- Maintains current camera rotation (or resets to perspective view)
- Handles edge cases: single point, degenerate bounds, very large objects
- Uses adaptive distance calculation based on object size
- Supports focus on lights and other non-geometric objects

#### `setCameraPreset(preset: CameraPreset, camera: THREE.Camera, controls: OrbitControls, target?: Vector3): void`

Sets camera to predefined view (Front, Side, Top, etc.).

```typescript
setCameraPreset("front", camera, controls, [0, 0, 0]);
setCameraPreset("top", camera, controls);
setCameraPreset("perspective", camera, controls); // Returns to current perspective
```

**Available Presets**:
- `"front"`: Camera looks at +Z axis (front view)
- `"back"`: Camera looks at -Z axis (back view)
- `"left"`: Camera looks at +X axis (left view)
- `"right"`: Camera looks at -X axis (right view)
- `"top"`: Camera looks down from +Y axis (top view)
- `"bottom"`: Camera looks up from -Y axis (bottom view)
- `"perspective"`: Current perspective view (no change)

#### `smoothCameraTransition(from: CameraState, to: CameraState, duration: number, mode: CameraSmoothingMode, onUpdate: (state: CameraState) => void, onCancel?: () => void): Promise<void>`

Smoothly animates camera between two states with configurable interpolation modes.

```typescript
await smoothCameraTransition(
  { position: [0, 0, 10], quaternion: [0, 0, 0, 1], target: [0, 0, 0], fov: 50, timestamp: 0 },
  { position: [5, 5, 5], quaternion: [0.1, 0.2, 0.3, 0.9], target: [0, 0, 0], fov: 60, timestamp: 0 },
  0.5, // duration in seconds
  "ease-in-out", // interpolation mode
  (state) => {
    // Update camera during transition
    setCameraPosition(state.position);
    setCameraQuaternion(state.quaternion);
    setCameraFov(state.fov);
  },
  () => {
    // Optional cancellation handler
    console.log("Transition cancelled");
  }
);
```

**UE5 Behavior** (Enhanced AAA-ready):
- Supports multiple interpolation modes: linear, ease-in-out, ease-in, ease-out, exponential, spring, instant
- Interpolates position using selected easing function
- Interpolates rotation using quaternion SLERP (spherical linear interpolation)
- Interpolates FOV smoothly (optional)
- Can be cancelled mid-transition with cancellation callback
- Frame-rate independent using delta time
- Handles rapid successive transitions gracefully
- Provides progress callback (0.0 to 1.0) for UI feedback

#### `checkCameraCollision(camera: THREE.Camera, position: [number, number, number], radius: number, scene: THREE.Scene): CollisionResult`

Checks if camera position collides with scene geometry.

```typescript
const result = checkCameraCollision(
  camera,
  [5, 5, 5],
  0.5, // collision radius
  scene
);

if (result.collided) {
  // Adjust position to avoid collision
  setCameraPosition(result.safePosition);
}
```

**Behavior**:
- Uses raycasting to detect collisions
- Supports configurable collision radius
- Returns safe position if collision detected
- Handles complex geometry and meshes
- Performance optimized with spatial partitioning (optional)

#### `applyCameraConstraints(camera: THREE.Camera, constraints: CameraConstraints): void`

Applies position and rotation constraints to camera.

```typescript
applyCameraConstraints(camera, {
  positionBounds: {
    min: [-10, 0, -10],
    max: [10, 20, 10]
  },
  rotationLimits: {
    minPitch: -80,
    maxPitch: 80,
    minYaw: -180,
    maxYaw: 180
  },
  distanceLimits: {
    min: 1,
    max: 100
  }
});
```

**Behavior**:
- Clamps camera position to bounds
- Limits rotation angles (pitch/yaw)
- Enforces distance limits from target
- Smooth constraint application (no snapping)
- Works with all camera modes

#### `calculateObjectBounds(objects: SceneObject[], includeChildren?: boolean): THREE.Box3`

Calculates combined bounding box of objects with high precision.

```typescript
const bounds = calculateObjectBounds(selectedObjects, true);
const center = bounds.getCenter(new THREE.Vector3());
const size = bounds.getSize(new THREE.Vector3());
```

**Behavior**:
- Handles empty selection gracefully
- Supports recursive child inclusion
- Accounts for object transforms
- Handles degenerate cases (points, lines)
- Returns world-space bounds

## Implementation Plan

### Phase 1: Core Infrastructure

1. **Add Camera State to Scene Store** (`src/stores/scene-store.ts`)
   - Add `cameraPosition`, `cameraTarget`, `cameraQuaternion`, `cameraBookmarks` to `SceneState`
   - Add camera settings: `cameraSpeed`, `cameraRotationSpeed`, `cameraZoomSpeed`, `cameraPanSpeed`, `cameraFocusDistance`
   - Add camera actions to `SceneActions`:
     - `focusOnSelection()` - Focus on selected objects (F key)
     - `setCameraPreset()` - Set camera to preset view
     - Camera speed setters
   - Implement actions with proper normalization
   - Add default values initialization using `CAMERA_DEFAULTS`

2. **Create Camera Utilities** (`src/utils/camera-utils.ts`)
   - Implement `quaternionFromEulerDegrees()`
   - Implement `eulerDegreesFromQuaternion()`
   - Implement `getCameraQuaternion()`
   - Implement `applyQuaternionToCamera()`
   - Implement `quaternionFromCameraLookAt()`
   - Add quaternion normalization helpers
   - **Camera Operations**:
     - `focusOnSelection()` - Calculate bounds and frame objects in view
     - `setCameraPreset()` - Set camera to Front/Side/Top/etc. views
     - `smoothCameraTransition()` - Animate camera between states with easing
     - `calculateObjectBounds()` - Calculate combined bounding box of objects

3. **Update Scene Persistence** (`src/utils/scene-persistence.ts`)
   - Add `cameraBookmarks` to `SceneFileData` interface
   - Update `serializeScene()` to include camera state
   - Update `deserializeScene()` to restore camera state

### Phase 2: OrbitControls Integration

1. **Add OrbitControls to Scene** (`src/components/scene/scene-renderer.tsx` or new component)
   - Import `<OrbitControls>` from `@react-three/drei`
   - Add to SceneRenderer component tree
   - Configure default settings (damping, zoom limits, etc.)

2. **Create Camera Sync Component** (`src/components/scene/camera-sync.tsx`)
   - Sync OrbitControls changes → Store (onChange handler)
   - Sync Store changes → OrbitControls (useEffect watching store)
   - Skip sync during fly mode (check fly mode state)
   - Handle F key for focus on selection
   - Handle number keys (1-9) for quick bookmark access
   - Handle Alt modifier for orbit/dolly/track modes
   - Custom Alt + LMB/RMB/MMB handlers (Maya/Unity-style controls)

### Phase 3: Fly Camera Controls

1. **Create Fly Camera Hook** (`src/hooks/use-fly-camera-controls.ts`)
   - Right mouse button detection
   - Mouse movement tracking (deltaX, deltaY)
   - Keyboard input handling (WASD + QE + RF + ZC + Numpad + Arrow keys)
   - Scroll wheel speed adjustment while RMB held
   - Frame-rate independent movement using `useFrame`
   - Global vs Local space vertical movement (E/Q vs R/F)
   - FOV adjustment with Z/C keys
   - Pitch clamping (±89°)

2. **Create Fly Camera Component** (`src/components/scene/fly-camera-controls.tsx`)
   - Use `useFlyCameraControls` hook
   - Disable OrbitControls during fly mode
   - Update camera target on exit
   - Handle input focus detection

### Phase 4: Views Tab UI

1. **Create Views Tab Component** (`src/components/views-tab.tsx`)
   - Camera position inputs (X, Y, Z)
   - Camera rotation inputs (RX, RY, RZ in degrees)
   - Camera speed settings (movement, rotation, zoom, pan)
   - Camera preset buttons (Front, Side, Top, etc.)
   - Focus button (alternative to F key)
   - Bookmark list UI with quick access (1-9 keys)
   - Add/Load/Rename/Delete bookmark actions
   - Real-time updates from store

2. **Integrate Views Tab into Sidebar**
   - Add "Views" tab to sidebar navigation
   - Ensure proper routing/display logic

### Phase 5: Extended Features

1. **Camera Collision System** (`src/utils/camera-collision.ts`)
   - Implement `checkCameraCollision()` function
   - Use THREE.Raycaster for collision detection
   - Support configurable collision radius
   - Return safe positions when collisions detected
   - Integrate with fly camera controls

2. **Camera Constraints System** (`src/utils/camera-constraints.ts`)
   - Implement `applyCameraConstraints()` function
   - Support position bounds (min/max)
   - Support rotation limits (pitch/yaw)
   - Support distance limits from target
   - Smooth constraint application

3. **Camera History System** (`src/utils/camera-history.ts`)
   - Implement undo/redo functionality
   - Store camera states in circular buffer (max 50)
   - Support keyboard shortcuts (Ctrl+Z/Ctrl+Y)
   - Integrate with store actions

4. **Enhanced Focus System** (`src/utils/camera-focus.ts`)
   - Enhanced `focusOnSelection()` with options
   - Padding support for better framing
   - Aspect ratio handling
   - Adaptive distance calculation
   - Handle edge cases (empty selection, single point, etc.)

5. **Viewport Overlays** (`src/components/scene/viewport-overlays.tsx`)
   - Grid overlay rendering
   - Guide lines (center, axes)
   - Safe area guides (16:9, 4:3, etc.)
   - Camera gizmo visualization
   - Frustum culling visualization

6. **Camera Gizmo** (`src/components/scene/camera-gizmo.tsx`)
   - Visual representation of camera
   - Shows position, rotation, FOV
   - Interactive manipulation (future)
   - Toggle visibility option

7. **Auto-Pivot System** (`src/components/scene/auto-pivot.tsx`)
   - Auto-update OrbitControls target on selection
   - Smooth pivot transitions
   - Configurable enable/disable
   - Works with multi-selection

### Phase 6: Performance & Optimization

1. **Performance Optimizations**
   - LOD-aware camera (adjust quality based on camera movement)
   - Frustum culling integration
   - Adaptive quality based on camera speed
   - Camera state caching
   - Debounce rapid manual input changes
   - Optimize quaternion operations (use THREE.js methods)

2. **Memory Management**
   - Limit camera history size (circular buffer)
   - Cleanup unused camera states
   - Efficient bookmark storage
   - Garbage collection for transitions

3. **Rendering Optimizations**
   - Conditional rendering based on camera movement
   - Skip updates when camera is stationary
   - Batch camera state updates
   - Use `useFrame` efficiently (avoid unnecessary work)

### Phase 7: Testing & Polish

1. **Test All Control Modes**
   - OrbitControls mode
   - Fly mode activation/deactivation
   - Manual input mode
   - Mode switching preserves state
   - Collision detection works correctly
   - Constraints are enforced properly

2. **Test Persistence**
   - Save/load scene with camera state
   - Bookmarks persist correctly
   - Camera history persists (optional)
   - Settings persist across sessions

3. **Test Extended Features**
   - Camera collision detection
   - Camera constraints
   - Camera history (undo/redo)
   - Enhanced focus options
   - Viewport overlays
   - Auto-pivot system

4. **Performance Testing**
   - Smooth camera updates (60fps+)
   - No frame drops during transitions
   - Efficient memory usage
   - Fast collision detection
   - Responsive UI during camera movement

5. **Edge Case Testing**
   - Empty scene
   - Single object
   - Very large objects
   - Very small objects
   - Objects at origin
   - Rapid camera movements
   - Multiple rapid focus operations

## Implementation Details

### Scene Canvas Integration

The `SceneCanvas` component (`src/components/scene-canvas.tsx`) will need:

1. **OrbitControls Component**: Add `<OrbitControls>` from `@react-three/drei` inside `<Canvas>`
2. **FlyCameraControls Component**: Add `<FlyCameraControls>` inside `<Canvas>`
3. **CameraQuaternionSync Component**: Add `<CameraQuaternionSync>` inside `<Canvas>` to sync manual changes
4. **Frameloop**: Current `frameloop="demand"` may need adjustment for smooth camera updates

### Fly Camera Controls Hook

The `useFlyCameraControls` hook (`src/hooks/use-fly-camera-controls.ts`) will provide:

- Right mouse button detection via `useEffect` + `addEventListener`
- Mouse movement tracking via `mousemove` events
- Keyboard input handling via `keydown`/`keyup` events
- Frame-rate independent movement using `useFrame` from `@react-three/fiber`
- Automatic OrbitControls disabling/enabling via `useThree().controls`
- Camera target update on fly mode exit

### Store Actions Implementation

Camera-related actions to implement in the scene store:

```typescript
// Set camera position (with optional history tracking)
setCameraPosition: (position: [number, number, number], addToHistory = false) => {
  // Check collision if enabled
  if (get().cameraCollisionEnabled) {
    const collisionResult = checkCameraCollision(
      camera,
      position,
      get().cameraCollisionRadius,
      scene
    );
    if (collisionResult.collided) {
      position = collisionResult.safePosition;
    }
  }
  
  // Apply constraints if set
  if (get().cameraConstraints) {
    position = applyConstraints(position, get().cameraConstraints);
  }
  
  set({ cameraPosition: position });
  
  // Add to history if requested
  if (addToHistory) {
    get().pushCameraState();
  }
  
  get().markSceneModified();
}

// Set camera target
setCameraTarget: (target: [number, number, number]) => {
  set({ cameraTarget: target });
  get().markSceneModified();
}

// Set camera quaternion (normalized automatically)
setCameraQuaternion: (quaternion: [number, number, number, number]) => {
  const q = new THREE.Quaternion(...quaternion);
  q.normalize();
  set({ cameraQuaternion: [q.x, q.y, q.z, q.w] });
  get().markSceneModified();
}

// Bookmark management
addCameraBookmark: (name: string) => {
  const state = get();
  const bookmark: CameraBookmark = {
    id: `bookmark_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
    name,
    position: [...state.cameraPosition],
    quaternion: [...state.cameraQuaternion],
    target: [...state.cameraTarget],
  };
  set({ cameraBookmarks: [...state.cameraBookmarks, bookmark] });
  get().markSceneModified();
}

deleteCameraBookmark: (id: string) => {
  set((state) => ({
    cameraBookmarks: state.cameraBookmarks.filter((b) => b.id !== id),
  }));
  get().markSceneModified();
}

renameCameraBookmark: (id: string, name: string) => {
  set((state) => ({
    cameraBookmarks: state.cameraBookmarks.map((b) =>
      b.id === id ? { ...b, name } : b
    ),
  }));
  get().markSceneModified();
}

loadCameraBookmark: (id: string) => {
  const state = get();
  const bookmark = state.cameraBookmarks.find((b) => b.id === id);
  if (bookmark) {
    set({
      cameraPosition: [...bookmark.position],
      cameraQuaternion: [...bookmark.quaternion],
      cameraTarget: bookmark.target ? [...bookmark.target] : state.cameraTarget,
    });
  }
},

// Camera control actions
focusOnSelection: () => {
  const state = get();
  const selectedObjects = state.objects.filter((obj) =>
    state.selectedObjectIds.includes(obj.id)
  );
  
  if (selectedObjects.length === 0) {
    // If no selection, focus on all objects
    const allObjects = state.objects;
    if (allObjects.length === 0) return;
    // Use camera-utils to calculate bounds and frame
    // This will be implemented in camera-utils.ts
  } else {
    // Focus on selected objects
    // Implementation in camera-utils.ts
  }
},

setCameraPreset: (preset: CameraPreset) => {
  // Implementation using camera-utils.ts
  // Sets camera to Front/Side/Top/etc. view
},

setCameraSpeed: (speed: number) => {
  set({ cameraSpeed: Math.max(0.1, Math.min(100, speed)) });
  get().markSceneModified();
},

setCameraRotationSpeed: (speed: number) => {
  set({ cameraRotationSpeed: Math.max(0.0001, Math.min(0.01, speed)) });
  get().markSceneModified();
},

setCameraZoomSpeed: (speed: number) => {
  set({ cameraZoomSpeed: Math.max(0.1, Math.min(10, speed)) });
  get().markSceneModified();
},

setCameraPanSpeed: (speed: number) => {
  set({ cameraPanSpeed: Math.max(0.1, Math.min(10, speed)) });
  get().markSceneModified();
},

setCameraFocusDistance: (distance: number) => {
  set({ cameraFocusDistance: Math.max(0.5, Math.min(5, distance)) });
  get().markSceneModified();
},

// Camera history actions
pushCameraState: () => {
  const state = get();
  const cameraState: CameraState = {
    position: [...state.cameraPosition],
    quaternion: [...state.cameraQuaternion],
    target: [...state.cameraTarget],
    fov: state.cameraFov,
    timestamp: Date.now(),
  };
  
  const history = [...state.cameraHistory];
  const index = state.cameraHistoryIndex;
  
  // Remove any states after current index (when undoing then making new change)
  const newHistory = history.slice(0, index + 1);
  
  // Add new state
  newHistory.push(cameraState);
  
  // Limit history size
  if (newHistory.length > CAMERA_DEFAULTS.MAX_HISTORY_STATES) {
    newHistory.shift();
  }
  
  set({
    cameraHistory: newHistory,
    cameraHistoryIndex: newHistory.length - 1,
  });
},

undoCameraState: () => {
  const state = get();
  if (state.cameraHistoryIndex > 0) {
    const prevState = state.cameraHistory[state.cameraHistoryIndex - 1];
    set({
      cameraPosition: [...prevState.position],
      cameraQuaternion: [...prevState.quaternion],
      cameraTarget: [...prevState.target],
      cameraFov: prevState.fov,
      cameraHistoryIndex: state.cameraHistoryIndex - 1,
    });
    get().markSceneModified();
  }
},

redoCameraState: () => {
  const state = get();
  if (state.cameraHistoryIndex < state.cameraHistory.length - 1) {
    const nextState = state.cameraHistory[state.cameraHistoryIndex + 1];
    set({
      cameraPosition: [...nextState.position],
      cameraQuaternion: [...nextState.quaternion],
      cameraTarget: [...nextState.target],
      cameraFov: nextState.fov,
      cameraHistoryIndex: state.cameraHistoryIndex + 1,
    });
    get().markSceneModified();
  }
},

clearCameraHistory: () => {
  set({
    cameraHistory: [],
    cameraHistoryIndex: -1,
  });
},

// Camera configuration actions
setCameraFov: (fov: number) => {
  set({ cameraFov: Math.max(10, Math.min(170, fov)) });
  get().markSceneModified();
},

setCameraSmoothingMode: (mode: CameraSmoothingMode) => {
  set({ cameraSmoothingMode: mode });
  get().markSceneModified();
},

setCameraSmoothingDuration: (duration: number) => {
  set({ cameraSmoothingDuration: Math.max(0, Math.min(5, duration)) });
  get().markSceneModified();
},

setCameraCollisionEnabled: (enabled: boolean) => {
  set({ cameraCollisionEnabled: enabled });
  get().markSceneModified();
},

setCameraCollisionRadius: (radius: number) => {
  set({ cameraCollisionRadius: Math.max(0.1, Math.min(5, radius)) });
  get().markSceneModified();
},

setCameraAutoPivot: (enabled: boolean) => {
  set({ cameraAutoPivot: enabled });
  get().markSceneModified();
},

setCameraConstraints: (constraints: CameraConstraints | null) => {
  set({ cameraConstraints: constraints });
  get().markSceneModified();
},

toggleViewportOverlay: (overlay: keyof ViewportOverlays) => {
  set((state) => ({
    viewportOverlays: {
      ...state.viewportOverlays,
      [overlay]: !state.viewportOverlays[overlay],
    },
  }));
  get().markSceneModified();
},
```

## Best Practices

### Quaternion Normalization

**Always normalize quaternions before storing**:

```typescript
const q = new THREE.Quaternion(...quaternion);
q.normalize();
// Store normalized quaternion
```

The `setCameraQuaternion` action should automatically normalize quaternions.

### Euler Angles for UI Only

- ✅ Use Euler angles for user input (degrees are intuitive)
- ✅ Convert Euler → Quaternion immediately on input
- ✅ Derive Euler from quaternion for display
- ❌ Never store Euler angles as canonical state
- ❌ Never use Euler angles for calculations

### Camera Bookmarks Best Practices

When creating bookmarks:

1. Always include position, quaternion, and optionally target
2. Use descriptive names for easy identification
3. Bookmarks are scene-specific and saved with scene files
4. Generate unique IDs using timestamp + random string
5. Include FOV in bookmark data for complete state restoration

### Camera History Best Practices

1. **When to Save History**:
   - Save state before major camera operations (focus, preset, bookmark load)
   - Save state after manual input changes (with debounce)
   - Don't save state during continuous movement (fly mode, orbit)

2. **History Management**:
   - Use circular buffer (max 50 states) to prevent memory issues
   - Clear history when loading new scene
   - Provide visual feedback for undo/redo availability

3. **Performance**:
   - Store only essential data (position, quaternion, target, fov)
   - Use shallow comparison to avoid duplicate states
   - Debounce rapid state saves

### Performance Considerations

- Use `useFrame` for frame-rate independent updates
- Avoid unnecessary re-renders by using Zustand selectors
- Quaternion comparisons should use `equals()` method for efficiency
- Manual input changes trigger immediate camera updates
- Debounce rapid manual input changes (100ms debounce recommended)
- Skip collision checks when collision is disabled
- Cache bounding box calculations for selected objects
- Use `requestAnimationFrame` for smooth transitions
- Batch multiple camera updates in single frame when possible

### React Three Fiber Integration

- Use `useThree()` hook to access camera and controls
- Use `useFrame()` for per-frame updates
- Access OrbitControls via `useThree().controls` (if using drei)
- Camera updates automatically trigger re-renders via R3F
- Use `useFrame` with priority for critical camera updates
- Consider using `useFrame` with `priority` parameter for collision checks

### Collision Detection Best Practices

1. **Performance**:
   - Only check collision when collision is enabled
   - Use spatial partitioning for large scenes (Octree/BVH)
   - Cache collision results when camera hasn't moved
   - Limit collision checks to visible objects only

2. **Accuracy**:
   - Use appropriate collision radius (0.5 units default)
   - Handle edge cases (no geometry, single point, etc.)
   - Provide fallback behavior when collision fails

### Constraint System Best Practices

1. **Application**:
   - Apply constraints smoothly (no snapping)
   - Validate constraints before applying
   - Provide visual feedback when constraints are active
   - Allow temporary constraint override (e.g., Shift key)

2. **User Experience**:
   - Make constraints optional and configurable
   - Show constraint boundaries in viewport (optional overlay)
   - Provide clear feedback when constraints are reached

### Tauri Considerations

- No special considerations needed for camera system
- Scene persistence already handles file I/O via Tauri plugins
- Camera state is serialized as JSON in scene files
- Consider compression for large camera history data

## Camera Default Values

```typescript
// Add to src/lib/defaults.ts
export const CAMERA_DEFAULTS = {
  POSITION: [5, 5, 5] as [number, number, number],
  TARGET: [0, 0, 0] as [number, number, number],
  QUATERNION: [0, 0, 0, 1] as [number, number, number, number], // Identity quaternion
  
  // Camera settings
  SPEED: 5.0,
  ROTATION_SPEED: 0.002,
  ZOOM_SPEED: 1.0,
  PAN_SPEED: 1.0,
  FOCUS_DISTANCE: 1.5,
  
  // Camera properties
  FOV: 50,
  NEAR: 0.1,
  FAR: 1000,
  
  // Smoothing
  SMOOTHING_MODE: "ease-in-out" as CameraSmoothingMode,
  SMOOTHING_DURATION: 0.3,
  
  // Collision
  COLLISION_ENABLED: false,
  COLLISION_RADIUS: 0.5,
  
  // Orbit controls
  ORBIT_DAMPING: true,
  ORBIT_DAMPING_FACTOR: 0.05,
  ORBIT_MIN_DISTANCE: 0.1,
  ORBIT_MAX_DISTANCE: 1000,
  ORBIT_MIN_POLAR_ANGLE: 0, // radians
  ORBIT_MAX_POLAR_ANGLE: Math.PI, // radians
  
  // Auto-pivot
  AUTO_PIVOT: true,
  
  // History
  MAX_HISTORY_STATES: 50,
  
  // Focus
  FOCUS_PADDING: 0.1,
  FOCUS_DURATION: 0.5,
  
  // Viewport overlays
  OVERLAY_GRID: true,
  OVERLAY_GUIDES: false,
  OVERLAY_SAFE_AREA: false,
  OVERLAY_CAMERA_GIZMO: true,
  OVERLAY_FRUSTUM_CULLING: false,
};
```

## Dependencies

The camera system will use:

- **@react-three/fiber**: Core R3F hooks (`useThree`, `useFrame`)
- **@react-three/drei**: `<OrbitControls>` component
- **three**: Quaternion math (`THREE.Quaternion`, `THREE.Euler`, `THREE.Vector3`)
- **zustand**: State management (existing scene store)
- **shadcn/ui**: UI components for Views tab (Input, Button, etc.)

## Integration Points

### Existing Systems

1. **Scene Store** (`src/stores/scene-store.ts`)
   - Add camera state and actions
   - Integrate with existing `markSceneModified()` pattern

2. **Scene Persistence** (`src/utils/scene-persistence.ts`)
   - Extend `SceneFileData` interface
   - Update serialization/deserialization

3. **Scene Canvas** (`src/components/scene-canvas.tsx`)
   - Add camera components to Canvas tree
   - May need to adjust `frameloop` mode

4. **Scene Renderer** (`src/components/scene/scene-renderer.tsx`)
   - Add OrbitControls component
   - Add camera sync components

5. **Sidebar Navigation**
   - Add "Views" tab (check existing sidebar structure)

## Future Extensibility

The quaternion-based architecture will enable:

- **Camera Animations**: Smooth interpolation between quaternions using `THREE.Quaternion.slerp()`
- **Cinematic Cameras**: Complex camera paths and transitions with keyframe support
- **Camera Constraints**: Limits on rotation without gimbal lock (already planned)
- **Multi-Camera Support**: Easy switching between camera states (multi-viewport foundation)
- **Camera Presets**: Predefined camera positions for common views (front, side, top, etc.)
- **Camera Sequences**: Record and playback camera movements
- **Camera Paths**: Bezier/spline-based camera paths for cinematic sequences
- **Camera Shake**: Post-processing effects for camera shake simulation
- **Camera Post-Processing**: Depth of field, motion blur, chromatic aberration
- **VR/AR Support**: Stereo camera setup for VR/AR rendering
- **Camera Layers**: Multiple camera layers for UI overlays
- **Camera Effects**: Lens flares, vignette, color grading per camera

## Accessibility Features

### Keyboard-Only Navigation

The camera system supports full keyboard-only navigation for accessibility:

- **Arrow Keys**: Move camera (Up/Down/Left/Right)
- **WASD**: Move camera in fly mode
- **QE**: Move up/down (Global space)
- **RF**: Move up/down (Local space)
- **Tab**: Cycle through camera modes
- **Enter**: Activate/deactivate fly mode
- **Space**: Focus on selection
- **1-9**: Load camera bookmarks
- **Ctrl+Z/Y**: Undo/redo camera movements

### Configurable Key Bindings

Users can customize all keyboard shortcuts:

```typescript
interface CameraKeyBindings {
  focus: string; // Default: "f"
  flyMode: string; // Default: "r" (right mouse button)
  bookmarks: string[]; // Default: ["1", "2", ..., "9"]
  undo: string; // Default: "ctrl+z"
  redo: string; // Default: "ctrl+y"
  // ... more bindings
}
```

### Visual Feedback

- **Fly Mode Indicator**: Visual indicator when fly mode is active
- **Constraint Warnings**: Visual feedback when constraints are reached
- **Collision Warnings**: Visual feedback when collision is detected
- **History Status**: Visual indicator for undo/redo availability

## Debugging Tools

### Camera Debug Overlay

Visual debugging tools for camera system:

```typescript
interface CameraDebugInfo {
  position: [number, number, number];
  rotation: [number, number, number]; // Euler angles
  target: [number, number, number];
  distance: number;
  fov: number;
  mode: "orbit" | "fly" | "manual";
  collision: boolean;
  constraints: boolean;
  fps: number;
  frameTime: number;
}
```

### Visual Debug Features

- **Camera Gizmo**: Visual representation of camera in viewport
- **Frustum Visualization**: Show camera frustum (viewing volume)
- **Collision Visualization**: Show collision sphere
- **Constraint Visualization**: Show constraint boundaries
- **Path Visualization**: Show camera movement path (for sequences)
- **Performance Metrics**: FPS, frame time, update frequency

### Debug Console Commands

```typescript
// Camera debug commands (via console or UI)
camera.debug.position([x, y, z]);
camera.debug.rotation([rx, ry, rz]);
camera.debug.fov(60);
camera.debug.mode("fly");
camera.debug.collision(true);
camera.debug.constraints({ min: [-10, 0, -10], max: [10, 20, 10] });
```

## Testing Checklist

### Basic Functionality
- [ ] Camera state initializes with defaults
- [ ] OrbitControls updates camera state correctly
- [ ] Fly mode activates/deactivates properly
- [ ] Manual input updates camera correctly
- [ ] Bookmarks save/load correctly
- [ ] Scene persistence includes camera state
- [ ] Mode switching preserves camera state
- [ ] Quaternions are always normalized
- [ ] Euler ↔ Quaternion conversions are accurate
- [ ] Performance is smooth (60fps+)

### Camera Features
- [ ] F key focuses on selected objects
- [ ] Focus calculates correct bounding box
- [ ] Camera presets (Front/Side/Top) work correctly
- [ ] Camera speed settings affect movement
- [ ] Camera rotation speed affects mouse sensitivity
- [ ] Camera zoom/pan speeds work correctly
- [ ] Smooth camera transitions animate properly
- [ ] Number keys (1-9) load bookmarks correctly
- [ ] OrbitControls damping provides smooth feel
- [ ] Pan/zoom behavior is responsive

### Extended Features
- [ ] Camera collision detection works correctly
- [ ] Camera constraints are enforced properly
- [ ] Camera history (undo/redo) works correctly
- [ ] Enhanced focus options work (padding, aspect ratio)
- [ ] Multiple smoothing modes work correctly
- [ ] Viewport overlays render correctly
- [ ] Auto-pivot updates on selection
- [ ] Performance optimizations are effective
- [ ] Edge cases are handled gracefully
- [ ] Keyboard-only navigation works
- [ ] Configurable key bindings work
- [ ] Debug tools display correct information

## Related Systems

- **Scene Persistence** (`src/utils/scene-persistence.ts`): Camera state will be saved with scene files
- **OrbitControls** (`@react-three/drei`): Provides interactive camera manipulation (orbit/pan/zoom)
- **Fly Camera Controls**: FPS-style navigation (Right Mouse + WASD)
- **Views Tab**: UI for camera management and bookmarks
- **Scene Store** (`src/stores/scene-store.ts`): Centralized state management for camera - to be extended
- **Camera Utilities**: Pure functions for quaternion operations and camera features - to be implemented
- **Selection System** (`src/stores/scene-store.ts`): Used for focus on selection (F key) - existing system

## Implementation Priority Guide

### Phase 1-4: Core Features (Essential)
These phases provide the foundation for professional camera controls:
- ✅ Basic camera state management
- ✅ OrbitControls integration
- ✅ Fly camera controls
- ✅ Views tab UI
- ✅ Camera bookmarks

### Extended Features Implementation Priority
These features enhance the camera system:
- 🔥 **High Priority**: Camera history (undo/redo), enhanced focus options
- ⚡ **Medium Priority**: Camera collision, camera constraints, viewport overlays
- 💡 **Low Priority**: Camera gizmo, auto-pivot (can be added incrementally)

### Performance Priority
Performance optimizations are essential for smooth operation:
- 🔥 **Critical**: Frame-rate independent updates, efficient quaternion operations
- ⚡ **Important**: Collision detection optimization, adaptive quality
- 💡 **Nice to Have**: LOD-aware camera, frustum culling integration

### Polish Priority
These features provide the final polish for a professional tool:
- 🔥 **High Priority**: Edge case handling, comprehensive testing
- ⚡ **Medium Priority**: Accessibility features, debug tools
- 💡 **Low Priority**: Visual feedback enhancements, extended overlays

## Summary of Enhancements

This implementation plan includes the following enhancements:

### 🎯 Core Enhancements
1. **Extended State Management**: Added camera history, FOV, constraints, collision settings, viewport overlays
2. **Enhanced Camera Operations**: Enhanced focus system with padding, aspect ratio handling, adaptive distance
3. **Multiple Smoothing Modes**: Linear, ease-in-out, exponential, spring physics, instant
4. **Camera History System**: Undo/redo support with circular buffer (max 50 states)
5. **Camera Collision Detection**: Prevent camera from going through geometry
6. **Camera Constraints**: Position bounds, rotation limits, distance limits

### 🚀 Performance Features
1. **Optimized Updates**: Frame-rate independent, efficient quaternion operations
2. **Collision Optimization**: Spatial partitioning support, caching
3. **Adaptive Quality**: LOD-aware camera, performance-based quality adjustment
4. **Memory Management**: Circular buffer for history, efficient state storage

### 🎨 User Experience
1. **Viewport Overlays**: Grid, guides, safe areas, camera gizmo, frustum visualization
2. **Auto-Pivot**: Automatic pivot updates on selection
3. **Visual Feedback**: Fly mode indicator, constraint warnings, collision warnings
4. **Accessibility**: Keyboard-only navigation, configurable key bindings

### 🛠️ Developer Tools
1. **Debug Overlay**: Camera position, rotation, mode, collision status, performance metrics
2. **Visual Debug**: Camera gizmo, frustum visualization, collision sphere, constraint boundaries
3. **Debug Console**: Commands for testing and debugging camera system

### 📋 Implementation Phases
The plan includes 7 phases:
- **Phase 1-4**: Core infrastructure
- **Phase 5**: Extended features
- **Phase 6**: Performance & optimization
- **Phase 7**: Testing & polish

### 🎓 Best Practices
Best practices section covering:
- Camera history management
- Collision detection best practices
- Constraint system best practices
- Performance optimization guidelines
- Memory management strategies

### 🔮 Future Extensibility
Future features list including:
- Camera sequences and paths
- Camera shake and post-processing
- VR/AR support
- Camera layers
- Camera effects (lens flares, vignette, color grading)

## Feature Summary

This implementation plan provides Rengine's camera system with professional viewport controls:

### Core Features
- **Perspective View Controls**: LMB/RMB/MMB + drag for look/pan/zoom
- **Fly Mode**: Right mouse + WASD + QE + RF + ZC for navigation
- **Focus on Selection**: F key focuses camera on selected objects
- **Orbit/Dolly/Track**: Alt + LMB/RMB/MMB for Maya/Unity-style controls
- **Camera Presets**: Front, Side, Top, Bottom views
- **Camera Bookmarks**: Save/load camera positions
- **Smooth Transitions**: Animated camera movements

### Settings & Controls
- **Camera Speed**: Adjustable movement speed with scroll wheel
- **Mouse Sensitivity**: Adjustable rotation sensitivity
- **Zoom/Pan Speed**: Separate controls for zoom and pan speeds
- **Focus Distance**: Configurable distance multiplier for focus operations
- **FOV Adjustment**: Z/C keys adjust field of view

### Keyboard Shortcuts
- **F Key**: Focus on selection
- **W/S/A/D**: Move forward/backward/left/right
- **Q/E**: Move down/up in Global space
- **R/F**: Move up/down in Local space
- **Z/C**: Raise/lower FOV
- **Numpad Keys**: Alternative inputs for all movement keys
- **Arrow Keys**: Alternative inputs for WASD
- **Page Up/Down**: Alternative inputs for E/Q
- **Scroll Wheel + RMB**: Adjust movement speed
- **1-9 Keys**: Quick bookmark access

### Behavior
- **Damping**: Smooth camera movements with damping
- **Zoom Limits**: Prevents camera from going too close/far
- **Pitch Clamping**: Prevents camera flipping
- **Mode Switching**: Seamless transitions between control modes
- **Global vs Local Space**: E/Q use world coordinates, R/F use camera-relative
- **Alt Modifier**: Enables orbit/dolly/track modes (Maya/Unity-style)

### Additional Features
- **Camera Collision**: Prevent camera from going through geometry
- **Camera Constraints**: Position and rotation limits
- **Camera History**: Undo/redo camera movements (Ctrl+Z/Ctrl+Y)
- **Enhanced Focus**: Padding, aspect ratio handling, adaptive distance
- **Multiple Smoothing Modes**: Linear, ease-in-out, exponential, spring physics
- **Viewport Overlays**: Grid, guides, safe areas, camera gizmo
- **Auto-Pivot**: Automatic pivot point updates on selection
- **Performance Optimizations**: LOD-aware, frustum culling, adaptive quality
- **Edge Case Handling**: Empty scenes, degenerate bounds, very large/small objects
- **Accessibility**: Keyboard-only navigation, configurable key bindings

## Architecture Enhancements

### Camera System Architecture

The camera system is designed with professional game engine principles:

1. **Separation of Concerns**:
   - State management (Zustand store)
   - Camera utilities (pure functions)
   - Camera controls (React components)
   - Camera effects (post-processing, optional)

2. **Performance First**:
   - Frame-rate independent updates
   - Efficient quaternion operations
   - Collision detection optimization
   - Adaptive quality based on performance

3. **Extensibility**:
   - Plugin-based architecture ready
   - Easy to add new camera modes
   - Support for custom interpolation modes
   - Foundation for cinematic camera system

4. **User Experience**:
   - Smooth transitions
   - Visual feedback
   - Undo/redo support
   - Configurable settings
   - Accessibility support

### Integration with Existing Systems

The camera system integrates seamlessly with:

- **Selection System**: Auto-pivot, focus on selection
- **Transform System**: Camera respects transform constraints
- **Scene Graph**: Camera aware of object hierarchy
- **Performance Monitor**: Adaptive quality based on FPS
- **Asset System**: Camera bookmarks saved with scenes
- **UI System**: Views tab, camera gizmo, overlays
