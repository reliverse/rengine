import {
  Box,
  Code,
  Globe,
  Image,
  Music,
  Package,
  Palette,
  Settings,
  Type,
} from "lucide-react";
import type { LucideIcon } from "lucide-react";

// Regex patterns for file path parsing
export const PATH_SEPARATOR_REGEX = /[/\\]/;
export const FILE_EXTENSION_REGEX = /\.(dff|obj|txd|col|ipl)$/i;

// Asset type configurations
export const ASSET_TYPES = {
  models: {
    label: "Models",
    icon: Box,
    accept: ".gltf,.glb,.obj,.fbx,.dae,.3ds,.stl,.ply",
    description: "3D models and meshes",
    color: "text-blue-500",
  },
  textures: {
    label: "Textures",
    icon: Image,
    accept: "image/*,.tga,.dds",
    description: "Image textures and materials",
    color: "text-green-500",
  },
  materials: {
    label: "Materials",
    icon: Palette,
    accept: ".json,.mat",
    description: "Material definitions and shaders",
    color: "text-purple-500",
  },
  audio: {
    label: "Audio",
    icon: Music,
    accept: ".wav,.mp3,.ogg,.aac,.flac",
    description: "Sound effects and music",
    color: "text-orange-500",
  },
  scripts: {
    label: "Scripts",
    icon: Code,
    accept: ".ts,.tsx,.js,.jsx,.lua,.py",
    description: "Game logic and behaviors",
    color: "text-yellow-500",
  },
  fonts: {
    label: "Fonts",
    icon: Type,
    accept: ".ttf,.otf,.woff,.woff2",
    description: "Typography and text rendering",
    color: "text-pink-500",
  },
  prefabs: {
    label: "Prefabs",
    icon: Package,
    accept: ".prefab,.json",
    description: "Reusable object templates",
    color: "text-indigo-500",
  },
  renderware: {
    label: "RW/GTA",
    icon: Settings,
    accept: ".img,.txd,.dff,.col,.ide,.pwn",
    description: "GTA engine assets and PWN scripts",
    color: "text-red-500",
  },
  remote: {
    label: "Remote",
    icon: Globe,
    accept: "",
    description: "Third-party asset libraries",
    color: "text-cyan-500",
  },
} as const satisfies Record<
  string,
  {
    label: string;
    icon: LucideIcon;
    accept: string;
    description: string;
    color: string;
  }
>;
