/**
 * Dark Theme System for Rengine
 * System-independent dark theme that works across all platforms
 */

export interface ThemeColors {
  // Background Colors
  BACKGROUND_PRIMARY: string;
  BACKGROUND_SECONDARY: string;
  BACKGROUND_TERTIARY: string;
  BACKGROUND_ACCENT: string;
  BACKGROUND_HOVER: string;
  BACKGROUND_SELECTED: string;
  BACKGROUND_DISABLED: string;

  // Text Colors
  TEXT_PRIMARY: string;
  TEXT_SECONDARY: string;
  TEXT_TERTIARY: string;
  TEXT_ACCENT: string;
  TEXT_SUCCESS: string;
  TEXT_WARNING: string;
  TEXT_ERROR: string;
  TEXT_DISABLED: string;
  TEXT_LINK: string;

  // Border Colors
  BORDER_PRIMARY: string;
  BORDER_SECONDARY: string;
  BORDER_TERTIARY: string;
  BORDER_ACCENT: string;
  BORDER_FOCUS: string;
  BORDER_ERROR: string;

  // Interactive Colors
  HOVER_COLOR: string;
  SELECTION_COLOR: string;
  FOCUS_COLOR: string;

  // Button Colors
  BUTTON_PRIMARY: string;
  BUTTON_SECONDARY: string;
  BUTTON_SUCCESS: string;
  BUTTON_WARNING: string;
  BUTTON_DANGER: string;
  BUTTON_DISABLED: string;

  BUTTON_HOVER: string;
  BUTTON_PRESSED: string;

  // Input Colors
  INPUT_BACKGROUND: string;
  INPUT_BORDER: string;
  INPUT_FOCUS: string;
  INPUT_ERROR: string;
  INPUT_DISABLED: string;

  // Status Colors
  STATUS_SUCCESS: string;
  STATUS_WARNING: string;
  STATUS_ERROR: string;
  STATUS_INFO: string;
  STATUS_NEUTRAL: string;

  // Special Colors
  SCROLLBAR_TRACK: string;
  SCROLLBAR_THUMB: string;
  TOOLTIP_BACKGROUND: string;
  TOOLTIP_TEXT: string;
  OVERLAY_BACKGROUND: string;
  SHADOW_COLOR: string;
}

// Professional Dark Theme Constants
export const ModernDarkTheme: ThemeColors = {
  // Background Colors
  BACKGROUND_PRIMARY: "#1e1e1e", // Main window background
  BACKGROUND_SECONDARY: "#252526", // Panel backgrounds
  BACKGROUND_TERTIARY: "#2d2d30", // Widget backgrounds
  BACKGROUND_ACCENT: "#37373d", // Accent backgrounds
  BACKGROUND_HOVER: "#3e3e42", // Hover states
  BACKGROUND_SELECTED: "#37373d", // Selected items
  BACKGROUND_DISABLED: "#3c3c3c", // Disabled backgrounds

  // Text Colors
  TEXT_PRIMARY: "#cccccc", // Main text
  TEXT_SECONDARY: "#969696", // Secondary text
  TEXT_TERTIARY: "#6b6b6b", // Tertiary text
  TEXT_ACCENT: "#007acc", // Links, highlights
  TEXT_SUCCESS: "#4ec9b0", // Success messages
  TEXT_WARNING: "#dcdcaa", // Warning messages
  TEXT_ERROR: "#f44747", // Error messages
  TEXT_DISABLED: "#696969", // Disabled text
  TEXT_LINK: "#3794ff", // Link color

  // Border Colors
  BORDER_PRIMARY: "#2d2d30", // Main borders
  BORDER_SECONDARY: "#464647", // Secondary borders
  BORDER_TERTIARY: "#5c5c5c", // Tertiary borders
  BORDER_ACCENT: "#007acc", // Accent borders
  BORDER_FOCUS: "#007acc", // Focus borders
  BORDER_ERROR: "#f44747", // Error borders

  // Interactive Colors
  HOVER_COLOR: "#3e3e42", // Hover states
  SELECTION_COLOR: "#37373d", // Selected items
  FOCUS_COLOR: "#007acc", // Focus indicators

  // Button Colors
  BUTTON_PRIMARY: "#0e639c", // Primary button
  BUTTON_SECONDARY: "#3c3c3c", // Secondary button
  BUTTON_SUCCESS: "#0f7b0f", // Success button
  BUTTON_WARNING: "#f9a825", // Warning button
  BUTTON_DANGER: "#d32f2f", // Danger button
  BUTTON_DISABLED: "#696969", // Disabled button

  BUTTON_HOVER: "#1177bb", // Button hover
  BUTTON_PRESSED: "#0a4f7a", // Button pressed

  // Input Colors
  INPUT_BACKGROUND: "#3c3c3c", // Input background
  INPUT_BORDER: "#464647", // Input border
  INPUT_FOCUS: "#007acc", // Input focus
  INPUT_ERROR: "#f44747", // Input error
  INPUT_DISABLED: "#2d2d30", // Input disabled

  // Status Colors
  STATUS_SUCCESS: "#4ec9b0", // Success status
  STATUS_WARNING: "#dcdcaa", // Warning status
  STATUS_ERROR: "#f44747", // Error status
  STATUS_INFO: "#3794ff", // Info status
  STATUS_NEUTRAL: "#969696", // Neutral status

  // Special Colors
  SCROLLBAR_TRACK: "#2d2d30", // Scrollbar track
  SCROLLBAR_THUMB: "#464647", // Scrollbar thumb
  TOOLTIP_BACKGROUND: "#2d2d30", // Tooltip background
  TOOLTIP_TEXT: "#cccccc", // Tooltip text
  OVERLAY_BACKGROUND: "rgba(0, 0, 0, 0.6)", // Overlay background
  SHADOW_COLOR: "rgba(0, 0, 0, 0.3)", // Shadow color
} as const;

// Light theme for future use (currently not used)
export const ModernLightTheme: ThemeColors = {
  // Background Colors
  BACKGROUND_PRIMARY: "#ffffff",
  BACKGROUND_SECONDARY: "#f8f8f8",
  BACKGROUND_TERTIARY: "#f0f0f0",
  BACKGROUND_ACCENT: "#e8e8e8",
  BACKGROUND_HOVER: "#e0e0e0",
  BACKGROUND_SELECTED: "#cce7ff",
  BACKGROUND_DISABLED: "#f5f5f5",

  // Text Colors
  TEXT_PRIMARY: "#1e1e1e",
  TEXT_SECONDARY: "#6b6b6b",
  TEXT_TERTIARY: "#969696",
  TEXT_ACCENT: "#0066cc",
  TEXT_SUCCESS: "#0f7b0f",
  TEXT_WARNING: "#f9a825",
  TEXT_ERROR: "#d32f2f",
  TEXT_DISABLED: "#cccccc",
  TEXT_LINK: "#0066cc",

  // Border Colors
  BORDER_PRIMARY: "#e0e0e0",
  BORDER_SECONDARY: "#cccccc",
  BORDER_TERTIARY: "#b0b0b0",
  BORDER_ACCENT: "#0066cc",
  BORDER_FOCUS: "#0066cc",
  BORDER_ERROR: "#d32f2f",

  // Interactive Colors
  HOVER_COLOR: "#e0e0e0",
  SELECTION_COLOR: "#cce7ff",
  FOCUS_COLOR: "#0066cc",

  // Button Colors
  BUTTON_PRIMARY: "#0066cc",
  BUTTON_SECONDARY: "#e0e0e0",
  BUTTON_SUCCESS: "#0f7b0f",
  BUTTON_WARNING: "#f9a825",
  BUTTON_DANGER: "#d32f2f",
  BUTTON_DISABLED: "#cccccc",

  BUTTON_HOVER: "#0056b3",
  BUTTON_PRESSED: "#004085",

  // Input Colors
  INPUT_BACKGROUND: "#ffffff",
  INPUT_BORDER: "#cccccc",
  INPUT_FOCUS: "#0066cc",
  INPUT_ERROR: "#d32f2f",
  INPUT_DISABLED: "#f5f5f5",

  // Status Colors
  STATUS_SUCCESS: "#0f7b0f",
  STATUS_WARNING: "#f9a825",
  STATUS_ERROR: "#d32f2f",
  STATUS_INFO: "#0066cc",
  STATUS_NEUTRAL: "#6b6b6b",

  // Special Colors
  SCROLLBAR_TRACK: "#f0f0f0",
  SCROLLBAR_THUMB: "#cccccc",
  TOOLTIP_BACKGROUND: "#2d2d30",
  TOOLTIP_TEXT: "#ffffff",
  OVERLAY_BACKGROUND: "rgba(0, 0, 0, 0.4)",
  SHADOW_COLOR: "rgba(0, 0, 0, 0.1)",
};

// Current active theme (can be switched in the future)
export const currentTheme = ModernDarkTheme;

// Theme utility functions
export function getThemeColor(key: keyof ThemeColors): string {
  return currentTheme[key];
}

export function getCssVariable(name: string): string {
  // Convert theme key to CSS variable format
  return `var(--theme-${name.toLowerCase().replace(/_/g, "-")})`;
}

// Generate CSS custom properties from theme
export function generateThemeCssVariables(): Record<string, string> {
  const variables: Record<string, string> = {};

  for (const [key, value] of Object.entries(currentTheme)) {
    const cssKey = `--theme-${key.toLowerCase().replace(/_/g, "-")}`;
    variables[cssKey] = value;
  }

  return variables;
}

// Theme-aware style helpers
export function createThemeStyles() {
  return {
    // Base styles using theme colors
    backgroundPrimary: `background-color: ${currentTheme.BACKGROUND_PRIMARY};`,
    backgroundSecondary: `background-color: ${currentTheme.BACKGROUND_SECONDARY};`,
    backgroundTertiary: `background-color: ${currentTheme.BACKGROUND_TERTIARY};`,

    textPrimary: `color: ${currentTheme.TEXT_PRIMARY};`,
    textSecondary: `color: ${currentTheme.TEXT_SECONDARY};`,
    textAccent: `color: ${currentTheme.TEXT_ACCENT};`,

    borderPrimary: `border: 1px solid ${currentTheme.BORDER_PRIMARY};`,
    borderSecondary: `border: 1px solid ${currentTheme.BORDER_SECONDARY};`,
    borderAccent: `border: 1px solid ${currentTheme.BORDER_ACCENT};`,

    // Button styles
    buttonPrimary: `
      background-color: ${currentTheme.BUTTON_PRIMARY};
      color: ${currentTheme.TEXT_PRIMARY};
      border: 1px solid ${currentTheme.BORDER_ACCENT};
    `,
    buttonHover: `
      background-color: ${currentTheme.BUTTON_HOVER};
    `,
    buttonPressed: `
      background-color: ${currentTheme.BUTTON_PRESSED};
    `,

    // Input styles
    input: `
      background-color: ${currentTheme.INPUT_BACKGROUND};
      color: ${currentTheme.TEXT_PRIMARY};
      border: 1px solid ${currentTheme.INPUT_BORDER};
    `,
    inputFocus: `
      border-color: ${currentTheme.INPUT_FOCUS};
    `,
    inputError: `
      border-color: ${currentTheme.INPUT_ERROR};
    `,

    // Hover and selection
    hover: `background-color: ${currentTheme.HOVER_COLOR};`,
    selected: `background-color: ${currentTheme.SELECTION_COLOR};`,
    focus: `border-color: ${currentTheme.FOCUS_COLOR};`,

    // Status colors
    success: `color: ${currentTheme.STATUS_SUCCESS};`,
    warning: `color: ${currentTheme.STATUS_WARNING};`,
    error: `color: ${currentTheme.STATUS_ERROR};`,
    info: `color: ${currentTheme.STATUS_INFO};`,
  };
}

// Theme-aware component style helpers
export function getButtonStyles(
  variant:
    | "primary"
    | "secondary"
    | "success"
    | "warning"
    | "danger" = "primary"
) {
  const base = `
    border: 1px solid;
    border-radius: 3px;
    padding: 4px 8px;
    font-size: 12px;
    cursor: pointer;
    transition: all 0.1s ease;
  `;

  switch (variant) {
    case "primary":
      return `
        ${base}
        background-color: ${currentTheme.BUTTON_PRIMARY};
        border-color: ${currentTheme.BORDER_ACCENT};
        color: ${currentTheme.TEXT_PRIMARY};
        &:hover { background-color: ${currentTheme.BUTTON_HOVER}; }
        &:active { background-color: ${currentTheme.BUTTON_PRESSED}; }
      `;
    case "secondary":
      return `
        ${base}
        background-color: ${currentTheme.BUTTON_SECONDARY};
        border-color: ${currentTheme.BORDER_SECONDARY};
        color: ${currentTheme.TEXT_PRIMARY};
        &:hover { background-color: ${currentTheme.HOVER_COLOR}; }
      `;
    case "success":
      return `
        ${base}
        background-color: ${currentTheme.BUTTON_SUCCESS};
        border-color: ${currentTheme.BUTTON_SUCCESS};
        color: white;
      `;
    case "warning":
      return `
        ${base}
        background-color: ${currentTheme.BUTTON_WARNING};
        border-color: ${currentTheme.BUTTON_WARNING};
        color: black;
      `;
    case "danger":
      return `
        ${base}
        background-color: ${currentTheme.BUTTON_DANGER};
        border-color: ${currentTheme.BUTTON_DANGER};
        color: white;
      `;
    default:
      return base;
  }
}

export function getInputStyles() {
  return `
    background-color: ${currentTheme.INPUT_BACKGROUND};
    color: ${currentTheme.TEXT_PRIMARY};
    border: 1px solid ${currentTheme.INPUT_BORDER};
    border-radius: 3px;
    padding: 4px 6px;
    font-size: 12px;
    transition: border-color 0.1s ease;

    &:focus {
      border-color: ${currentTheme.INPUT_FOCUS};
      outline: none;
    }

    &:disabled {
      background-color: ${currentTheme.INPUT_DISABLED};
      color: ${currentTheme.TEXT_DISABLED};
      cursor: not-allowed;
    }

    &.error {
      border-color: ${currentTheme.INPUT_ERROR};
    }
  `;
}

export function getPanelStyles() {
  return `
    background-color: ${currentTheme.BACKGROUND_SECONDARY};
    color: ${currentTheme.TEXT_PRIMARY};
    border: 1px solid ${currentTheme.BORDER_SECONDARY};
    border-radius: 3px;
  `;
}

export function getScrollbarStyles() {
  return `
    &::-webkit-scrollbar {
      width: 12px;
      height: 12px;
    }

    &::-webkit-scrollbar-track {
      background: ${currentTheme.SCROLLBAR_TRACK};
      border-radius: 6px;
    }

    &::-webkit-scrollbar-thumb {
      background: ${currentTheme.SCROLLBAR_THUMB};
      border-radius: 6px;

      &:hover {
        background: ${currentTheme.BORDER_ACCENT};
      }
    }

    &::-webkit-scrollbar-corner {
      background: ${currentTheme.BACKGROUND_PRIMARY};
    }
  `;
}

// Export type for use in components
export type ThemeKey = keyof ThemeColors;
