import * as THREE from "three";

// Helper function to create Three.js mesh from DFF geometry data
export function createMeshFromDffData(dffResult: any): THREE.Group {
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
          1.0 - uv.v,
        ]); // Flip V for Three.js
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

  return group;
}
