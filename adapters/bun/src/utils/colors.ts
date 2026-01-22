import pc from "picocolors";

/**
 * Beautiful color scheme for Rengine CLI and converter output
 * Uses picocolors for consistent, accessible terminal colors
 */

// Error colors - for failures and critical issues
export const colors = {
  // Error states
  error: pc.red,
  errorBright: pc.redBright,

  // Success states
  success: pc.green,
  successBright: pc.greenBright,

  // Warning states
  warning: pc.yellow,
  warningBright: pc.yellowBright,
  caution: pc.magenta, // Using magenta as caution color
  cautionBright: pc.magentaBright,

  // Info states
  info: pc.blue,
  infoBright: pc.blueBright,

  // File and process states
  file: pc.cyan,
  fileBright: pc.cyanBright,
  process: pc.magenta,
  processBright: pc.magentaBright,

  // Data and texture states
  texture: pc.yellow, // Using yellow for texture (distinct from warning)
  textureBright: pc.yellowBright,
  geometry: pc.blue, // Using blue for geometry (distinct from info)
  geometryBright: pc.blueBright,

  // Neutral colors
  neutral: pc.gray,
  neutralBright: pc.white,

  // Special accent colors
  accent: pc.magentaBright,
  highlight: pc.cyanBright,
  subtle: pc.gray,
};

// Semantic color aliases for specific use cases
export const semanticColors = {
  // CLI specific colors
  cli: {
    error: colors.error,
    success: colors.success,
    warning: colors.warning,
    info: colors.info,
    file: colors.file,
    process: colors.process,
  },

  // Converter specific colors
  converter: {
    processing: colors.process,
    texture: colors.texture,
    geometry: colors.geometry,
    success: colors.success,
    warning: colors.warning,
    error: colors.error,
  },

  // Progress indicators
  progress: {
    start: colors.infoBright,
    complete: colors.successBright,
    partial: colors.warningBright,
    failed: colors.errorBright,
  },

  // File types
  files: {
    dff: colors.fileBright,
    txd: colors.textureBright,
    gltf: colors.geometryBright,
    glb: colors.successBright,
  },
};

// Color combinations for rich output
export const colorCombos = {
  // Success with highlights
  successHighlight: (text: string) =>
    `${colors.success("✓")} ${colors.neutralBright(text)}`,
  successFile: (text: string) =>
    `${colors.success("✓")} ${colors.fileBright(text)}`,

  // Error with details
  errorDetail: (text: string, detail: string) =>
    `${colors.error("✗")} ${colors.neutralBright(text)} ${colors.neutral(`(${detail})`)}`,

  // Warning with suggestions
  warningSuggest: (text: string, suggestion: string) =>
    `${colors.warning("⚠")} ${colors.neutralBright(text)}\n${colors.neutral(`   → ${suggestion}`)}`,

  // Info messages
  info: (symbol: string, message: string) =>
    `${colors.info(symbol)} ${colors.neutralBright(message)}`,

  // Processing with progress
  processing: (text: string, count?: string) =>
    `${colors.process("⟳")} ${colors.neutralBright(text)}${count ? ` ${colors.info(`(${count})`)}` : ""}`,

  // File processing
  fileProcessing: (filename: string, action: string) =>
    `${colors.file("📁")} ${colors.fileBright(filename)} ${colors.neutral(`→ ${action}`)}`,

  // Texture processing
  textureProcessing: (textureName: string, status: string) =>
    `${colors.texture("🖼️")} ${colors.textureBright(textureName)} ${colors.neutral(`→ ${status}`)}`,
};

// Utility functions for consistent formatting
export const formatters = {
  // Error messages
  error: (symbol: string, message: string) =>
    `${colors.error(symbol)} ${colors.neutralBright(message)}`,

  // Success messages
  success: (symbol: string, message: string) =>
    `${colors.success(symbol)} ${colors.neutralBright(message)}`,

  // Info messages
  info: (symbol: string, message: string) =>
    `${colors.info(symbol)} ${colors.neutralBright(message)}`,

  // Warning messages
  warning: (symbol: string, message: string) =>
    `${colors.warning(symbol)} ${colors.neutralBright(message)}`,

  // File paths
  file: (filepath: string) => colors.fileBright(filepath),

  // Numbers and counts
  count: (num: number, unit: string) =>
    `${colors.infoBright(num.toString())} ${colors.neutral(unit)}`,

  // Time duration
  duration: (seconds: number) => `${colors.successBright(seconds.toFixed(1))}s`,

  // Progress counters
  progress: (current: number, total: number) =>
    `${colors.infoBright(current.toString())}${colors.neutral("/")}${colors.neutralBright(total.toString())}`,
};
