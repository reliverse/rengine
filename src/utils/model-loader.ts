import * as THREE from "three";

// Helper function to parse CSV line respecting quoted fields
function parseCSVLine(line: string): string[] {
  const result: string[] = [];
  let current = "";
  let inQuotes = false;

  for (const char of line) {
    if (char === '"') {
      inQuotes = !inQuotes;
    } else if (char === "," && !inQuotes) {
      result.push(current.trim());
      current = "";
    } else {
      current += char;
    }
  }

  // Don't forget the last field
  result.push(current.trim());

  return result;
}

// Model loader utility for loading 3D models by SA:MP model ID from CSV data
export interface ModelData {
  id: number;
  radius: number;
  name: string;
  hasCollision: boolean;
  breaksOnHit: boolean;
  visibleByTime: string;
  hasAnimation: boolean;
  borderBoxLength: number;
  borderBoxWidth: number;
  borderBoxHeight: number;
  materialsFile: string;
  definitionFile: string;
  modelFile: string;
  tags: string;
}

let csvDataCache: ModelData[] | null = null;

// Model cache to store loaded models and avoid re-processing the same DFF files
const modelCache = new Map<
  number,
  {
    modelData: ModelData;
    importedModel: THREE.Group;
    initialScale: number;
  }
>();

// Deep clone a THREE.Group with all its meshes, geometries, and materials
export function deepCloneModel(source: THREE.Group): THREE.Group {
  const cloned = new THREE.Group();
  cloned.name = source.name;
  cloned.position.copy(source.position);
  cloned.rotation.copy(source.rotation);
  cloned.scale.copy(source.scale);

  // Clone each direct child
  for (const child of source.children) {
    if (child instanceof THREE.Mesh) {
      // Clone geometry (shared is fine for performance)
      const clonedGeometry = child.geometry;

      // Clone material (important - each instance needs its own material)
      let clonedMaterial: THREE.Material | THREE.Material[];
      if (Array.isArray(child.material)) {
        clonedMaterial = child.material.map((mat) => mat.clone());
      } else {
        clonedMaterial = child.material.clone();
      }

      // Create new mesh with cloned material
      const clonedMesh = new THREE.Mesh(clonedGeometry, clonedMaterial);
      clonedMesh.name = child.name;
      clonedMesh.position.copy(child.position);
      clonedMesh.rotation.copy(child.rotation);
      clonedMesh.scale.copy(child.scale);
      clonedMesh.castShadow = child.castShadow;
      clonedMesh.receiveShadow = child.receiveShadow;
      clonedMesh.visible = child.visible;

      cloned.add(clonedMesh);
    } else if (child instanceof THREE.Group) {
      // Recursively clone nested groups
      const clonedGroup = deepCloneModel(child);
      cloned.add(clonedGroup);
    } else {
      // For other object types, use default clone
      cloned.add(child.clone(true));
    }
  }

  return cloned;
}

// Load and parse the GTA SA CSV file
export async function loadCsvData(): Promise<ModelData[]> {
  if (csvDataCache) {
    return csvDataCache;
  }

  try {
    // Load the CSV file from the public directory
    const response = await fetch("/preloaded/gta-sa.csv");
    if (!response.ok) {
      throw new Error(`Failed to load CSV file: ${response.statusText}`);
    }

    const csvText = await response.text();
    const lines = csvText.split("\n").filter((line) => line.trim());

    // Skip header line and parse data
    const data: ModelData[] = [];
    for (let i = 1; i < lines.length; i++) {
      const line = lines[i].trim();
      if (!line) continue;

      // Use proper CSV parser that handles quoted fields (e.g., "No, visible always")
      const parts = parseCSVLine(line);
      if (parts.length >= 13) {
        data.push({
          id: Number.parseInt(parts[0], 10),
          radius: Number.parseFloat(parts[1]),
          name: parts[2],
          hasCollision: parts[3] === "Yes",
          breaksOnHit: parts[4] === "Yes",
          visibleByTime: parts[5],
          hasAnimation: parts[6] === "Yes",
          borderBoxLength: Number.parseFloat(parts[7]),
          borderBoxWidth: Number.parseFloat(parts[8]),
          borderBoxHeight: Number.parseFloat(parts[9]),
          materialsFile: parts[10],
          definitionFile: parts[11],
          modelFile: parts[12],
          tags: parts.slice(13).join(","),
        });
      }
    }

    csvDataCache = data;
    return data;
  } catch (error) {
    console.error("Failed to load CSV data:", error);
    throw error;
  }
}

// Get model data by model ID
export async function getModelDataById(
  modelId: number
): Promise<ModelData | null> {
  const data = await loadCsvData();
  return data.find((model) => model.id === modelId) || null;
}

// Load a 3D model by model ID
export async function loadModelById(modelId: number): Promise<{
  modelData: ModelData;
  importedModel: THREE.Group;
  initialScale: number;
} | null> {
  // Check cache first
  const cached = modelCache.get(modelId);
  if (cached) {
    // Deep clone the model for reuse (so each instance is independent)
    return {
      modelData: cached.modelData,
      importedModel: deepCloneModel(cached.importedModel),
      initialScale: cached.initialScale,
    };
  }

  const modelData = await getModelDataById(modelId);
  if (!modelData) {
    console.warn(`Model with ID ${modelId} not found in CSV data`);
    return null;
  }

  // Check if it's a DFF file
  if (!modelData.modelFile.endsWith(".dff")) {
    console.warn(
      `Model ${modelData.name} (${modelId}) is not a DFF file: ${modelData.modelFile}`
    );
    return null;
  }

  // Try to load the actual DFF file first
  try {
    // Import Tauri API
    const { invoke } = await import("@tauri-apps/api/core");

    // Get home directory from Tauri backend (browser doesn't have process.env)
    let homeDir = "";
    try {
      homeDir = (await invoke("get_home_directory")) as string;
    } catch {
      homeDir = "/home/user"; // Fallback
    }

    // Try to load DFF from the preloaded resources directory
    // Primary location: user's preloaded resources folder
    const possiblePaths = [
      `${homeDir}/Documents/reliverse/rengine/preloaded/resources/gta-sa/${modelData.modelFile}`,
      // Fallback paths if needed
      `${homeDir}/.wine/drive_c/Program Files/Rockstar Games/GTA San Andreas/models/${modelData.modelFile}`,
      `${homeDir}/.steam/steam/steamapps/common/Grand Theft Auto San Andreas/models/${modelData.modelFile}`,
    ];

    let dffLoaded = false;
    let result: any = null;

    for (const filePath of possiblePaths) {
      try {
        result = (await invoke("import_dff_file", { filePath })) as any;
        dffLoaded = true;
        break;
      } catch {
        // Path not found, try next
      }
    }

    if (dffLoaded && result) {
      // Create Three.js mesh from DFF geometry data
      const importedModel = createMeshFromDffData(result);

      // Calculate initial scale to fit the model reasonably in the scene
      const box = new THREE.Box3().setFromObject(importedModel);
      const size = box.getSize(new THREE.Vector3());
      const maxDimension = Math.max(size.x, size.y, size.z);
      const initialScale =
        maxDimension > 0 ? Math.min(10 / maxDimension, 1) : 1;

      // Apply initial scale
      importedModel.scale.setScalar(initialScale);

      const loadResult = {
        modelData,
        importedModel,
        initialScale,
      };

      // Cache the loaded model for reuse
      modelCache.set(modelId, loadResult);

      // Return a deep clone so the cached version stays pristine
      return {
        modelData: loadResult.modelData,
        importedModel: deepCloneModel(loadResult.importedModel),
        initialScale: loadResult.initialScale,
      };
    }
  } catch (error) {
    console.warn(
      `Failed to load DFF file for ${modelData.name} (${modelId}), falling back to placeholder:`,
      error
    );
  }

  // Fallback: Create a placeholder box based on bounding box dimensions
  console.log(
    `Creating placeholder geometry for ${modelData.name} (${modelId})`
  );
  const importedModel = createPlaceholderFromBoundingBox(modelData);

  // Calculate initial scale for placeholder
  const maxDimension = Math.max(
    modelData.borderBoxLength,
    modelData.borderBoxWidth,
    modelData.borderBoxHeight
  );
  const initialScale = maxDimension > 0 ? Math.min(10 / maxDimension, 1) : 1;

  // Apply initial scale
  importedModel.scale.setScalar(initialScale);

  const loadResult = {
    modelData,
    importedModel,
    initialScale,
  };

  // Cache the placeholder model for reuse
  modelCache.set(modelId, loadResult);

  // Return a deep clone so the cached version stays pristine
  return {
    modelData: loadResult.modelData,
    importedModel: deepCloneModel(loadResult.importedModel),
    initialScale: loadResult.initialScale,
  };
}

// Batch load multiple models in parallel (with concurrency limit)
export async function loadModelsByIdsBatch(
  modelIds: number[],
  batchSize = 20
): Promise<
  Map<
    number,
    {
      modelData: ModelData;
      importedModel: THREE.Group;
      initialScale: number;
    } | null
  >
> {
  const results = new Map<
    number,
    {
      modelData: ModelData;
      importedModel: THREE.Group;
      initialScale: number;
    } | null
  >();

  // Process in batches to avoid overwhelming the system
  for (let i = 0; i < modelIds.length; i += batchSize) {
    const batch = modelIds.slice(i, i + batchSize);

    // Load batch in parallel
    const batchResults = await Promise.all(
      batch.map(async (modelId) => {
        try {
          const result = await loadModelById(modelId);
          return { modelId, result };
        } catch (error) {
          console.error(`Failed to load model ${modelId}:`, error);
          return { modelId, result: null };
        }
      })
    );

    // Store results
    for (const { modelId, result } of batchResults) {
      results.set(modelId, result);
    }
  }

  return results;
}

// Helper function to create Three.js mesh from DFF geometry data
function createMeshFromDffData(dffResult: any): THREE.Group {
  const group = new THREE.Group();

  if (!dffResult.geometries || dffResult.geometries.length === 0) {
    // Return empty group if no geometries
    return group;
  }

  dffResult.geometries.forEach((geometry: any, geomIndex: number) => {
    try {
      const threeGeometry = new THREE.BufferGeometry();

      // Convert vertices
      if (geometry.vertices && geometry.vertices.length > 0) {
        const positions = geometry.vertices.flatMap((v: any) => [
          v.x,
          v.y,
          v.z,
        ]);
        threeGeometry.setAttribute(
          "position",
          new THREE.Float32BufferAttribute(positions, 3)
        );
      }

      // Convert normals if available
      if (geometry.normals && geometry.normals.length > 0) {
        const normals = geometry.normals.flatMap((v: any) => [v.x, v.y, v.z]);
        threeGeometry.setAttribute(
          "normal",
          new THREE.Float32BufferAttribute(normals, 3)
        );
      }

      // Convert UV coordinates if available
      if (
        geometry.uv_layers &&
        geometry.uv_layers.length > 0 &&
        geometry.uv_layers[0].length > 0
      ) {
        const uvs = geometry.uv_layers[0].flatMap((uv: any) => [
          uv.u,
          1.0 - uv.v, // Flip V for Three.js
        ]);
        threeGeometry.setAttribute(
          "uv",
          new THREE.Float32BufferAttribute(uvs, 2)
        );
      }

      // Convert triangles to indices
      if (geometry.triangles && geometry.triangles.length > 0) {
        const indices = geometry.triangles.flatMap((t: any) => [t.a, t.b, t.c]);
        threeGeometry.setIndex(indices);
      }

      // Compute normals if not provided
      if (!geometry.normals || geometry.normals.length === 0) {
        threeGeometry.computeVertexNormals();
      }

      threeGeometry.computeBoundingSphere();
      threeGeometry.computeBoundingBox();

      // Create material
      let material: THREE.Material;
      if (geometry.materials && geometry.materials.length > 0) {
        const dffMaterial = geometry.materials[0];
        const color = new THREE.Color(
          dffMaterial.color.r / 255,
          dffMaterial.color.g / 255,
          dffMaterial.color.b / 255
        );

        // NOTE: TXD textures are NOT loaded here - only vertex colors!
        material = new THREE.MeshStandardMaterial({
          color,
          transparent: dffMaterial.color.a < 255,
          opacity: dffMaterial.color.a / 255,
          roughness: 0.7,
          metalness: 0.1,
          side: THREE.DoubleSide,
        });
      } else {
        material = new THREE.MeshStandardMaterial({
          color: 0xcc_cc_cc,
          roughness: 0.7,
          metalness: 0.1,
          side: THREE.DoubleSide,
        });
      }

      const mesh = new THREE.Mesh(threeGeometry, material);
      mesh.castShadow = true;
      mesh.receiveShadow = true;
      mesh.name = `geometry_${geomIndex}`;

      group.add(mesh);
    } catch (err) {
      console.warn(`Failed to create geometry ${geomIndex}:`, err);
    }
  });

  // Center the model
  const box = new THREE.Box3().setFromObject(group);
  const center = box.getCenter(new THREE.Vector3());
  group.position.sub(center);

  return group;
}

// Create a placeholder box geometry based on bounding box dimensions from CSV
function createPlaceholderFromBoundingBox(modelData: ModelData): THREE.Group {
  const group = new THREE.Group();

  // Create a box geometry with the bounding box dimensions
  const geometry = new THREE.BoxGeometry(
    modelData.borderBoxLength,
    modelData.borderBoxHeight,
    modelData.borderBoxWidth
  );

  // Create a material with a distinctive color to indicate it's a placeholder
  const material = new THREE.MeshStandardMaterial({
    color: 0xff_6b_6b, // Reddish color to indicate placeholder
    roughness: 0.7,
    metalness: 0.1,
    transparent: true,
    opacity: 0.8,
  });

  const mesh = new THREE.Mesh(geometry, material);
  mesh.castShadow = true;
  mesh.receiveShadow = true;
  mesh.name = `placeholder_${modelData.name}`;

  // Add wireframe overlay to make it clear this is a placeholder
  const wireframeMaterial = new THREE.MeshBasicMaterial({
    color: 0x00_00_00,
    wireframe: true,
  });
  const wireframeMesh = new THREE.Mesh(geometry, wireframeMaterial);
  mesh.add(wireframeMesh);

  group.add(mesh);

  // Add a label or indicator that this is a placeholder
  // (Three.js text rendering would require additional setup, so we'll skip for now)

  return group;
}
