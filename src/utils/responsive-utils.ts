/**
 * Responsive Design Utilities for Rengine
 * Based on RWMS responsive design system with DPI awareness
 */

export interface ResponsiveConfig {
  breakpoint: "small" | "medium" | "large" | "xlarge";
  scaleFactor: number;
  dpi: number;
  screenWidth: number;
  screenHeight: number;
  isHighDPI: boolean;
}

export interface FontConfig {
  header: { size: number; weight: string };
  subheader: { size: number; weight: string };
  body: { size: number; weight: string };
  small: { size: number; weight: string };
  code: { size: number; weight: string; family: string };
  menu: { size: number; weight: string };
  status: { size: number; weight: string };
}

export interface SpacingConfig {
  small: number;
  medium: number;
  large: number;
  xlarge: number;
  xxlarge: number;
}

export interface ComponentSize {
  button: { width: number; height: number };
  input: { height: number };
  panel: { minWidth: number; maxWidth: number };
  dialog: { minWidth: number; maxWidth: number };
}

class ResponsiveManager {
  private config: ResponsiveConfig;
  private fontConfig: FontConfig;
  private spacingConfig: SpacingConfig;
  private componentSize: ComponentSize;

  constructor() {
    this.config = this.detectResponsiveConfig();
    this.fontConfig = this.calculateFontConfig();
    this.spacingConfig = this.calculateSpacingConfig();
    this.componentSize = this.calculateComponentSizes();

    // Update on window resize
    if (typeof window !== "undefined") {
      window.addEventListener("resize", () => {
        this.updateConfig();
      });
    }
  }

  private detectResponsiveConfig(): ResponsiveConfig {
    if (typeof window === "undefined") {
      return {
        breakpoint: "large",
        scaleFactor: 1.0,
        dpi: 96,
        screenWidth: 1920,
        screenHeight: 1080,
        isHighDPI: false,
      };
    }

    const screenWidth = window.innerWidth;
    const screenHeight = window.innerHeight;

    // Detect DPI (device pixel ratio)
    const dpi = window.devicePixelRatio * 96; // Base DPI is 96
    const isHighDPI = dpi > 120;

    // Determine breakpoint
    let breakpoint: ResponsiveConfig["breakpoint"];
    let scaleFactor: number;

    if (screenWidth < 768) {
      breakpoint = "small";
      scaleFactor = 0.8;
    } else if (screenWidth < 1024) {
      breakpoint = "medium";
      scaleFactor = 0.9;
    } else if (screenWidth < 1440) {
      breakpoint = "large";
      scaleFactor = 1.0;
    } else {
      breakpoint = "xlarge";
      scaleFactor = 1.1;
    }

    // Adjust scale factor for high DPI displays
    if (isHighDPI) {
      scaleFactor *= Math.min(dpi / 96, 2.0); // Cap at 2x scaling
    }

    return {
      breakpoint,
      scaleFactor,
      dpi,
      screenWidth,
      screenHeight,
      isHighDPI,
    };
  }

  private updateConfig(): void {
    const newConfig = this.detectResponsiveConfig();
    const configChanged =
      JSON.stringify(this.config) !== JSON.stringify(newConfig);

    if (configChanged) {
      this.config = newConfig;
      this.fontConfig = this.calculateFontConfig();
      this.spacingConfig = this.calculateSpacingConfig();
      this.componentSize = this.calculateComponentSizes();

      // Dispatch custom event for components to react to responsive changes
      if (typeof window !== "undefined") {
        window.dispatchEvent(
          new CustomEvent("responsive-change", {
            detail: {
              config: this.config,
              fontConfig: this.fontConfig,
              spacingConfig: this.spacingConfig,
              componentSize: this.componentSize,
            },
          })
        );
      }
    }
  }

  private calculateFontConfig(): FontConfig {
    const baseSize = 14;
    const scaledBase = baseSize * this.config.scaleFactor;

    return {
      header: {
        size: Math.round(scaledBase * 1.5),
        weight: "bold",
      },
      subheader: {
        size: Math.round(scaledBase * 1.25),
        weight: "bold",
      },
      body: {
        size: Math.round(scaledBase),
        weight: "normal",
      },
      small: {
        size: Math.round(scaledBase * 0.875),
        weight: "normal",
      },
      code: {
        size: Math.round(scaledBase * 0.9),
        weight: "normal",
        family: 'Monaco, Menlo, "Ubuntu Mono", monospace',
      },
      menu: {
        size: Math.round(scaledBase * 0.9),
        weight: "normal",
      },
      status: {
        size: Math.round(scaledBase * 0.8),
        weight: "normal",
      },
    };
  }

  private calculateSpacingConfig(): SpacingConfig {
    const baseSpacing = 4;
    const scale = this.config.scaleFactor;

    return {
      small: Math.round(baseSpacing * scale),
      medium: Math.round(baseSpacing * 1.5 * scale),
      large: Math.round(baseSpacing * 2.5 * scale),
      xlarge: Math.round(baseSpacing * 3.5 * scale),
      xxlarge: Math.round(baseSpacing * 5 * scale),
    };
  }

  private calculateComponentSizes(): ComponentSize {
    const scale = this.config.scaleFactor;

    return {
      button: {
        width: Math.round(80 * scale),
        height: Math.round(28 * scale),
      },
      input: {
        height: Math.round(28 * scale),
      },
      panel: {
        minWidth: Math.round(300 * scale),
        maxWidth: Math.round(600 * scale),
      },
      dialog: {
        minWidth: Math.round(400 * scale),
        maxWidth: Math.round(800 * scale),
      },
    };
  }

  // Public API methods
  getConfig(): ResponsiveConfig {
    return { ...this.config };
  }

  getFontConfig(): FontConfig {
    return { ...this.fontConfig };
  }

  getSpacingConfig(): SpacingConfig {
    return { ...this.spacingConfig };
  }

  getComponentSize(): ComponentSize {
    return { ...this.componentSize };
  }

  getScaledSize(baseSize: number): number {
    return Math.round(baseSize * this.config.scaleFactor);
  }

  getWindowSize(): [number, number] {
    const scale = this.config.scaleFactor;
    const baseWidth = 1200;
    const baseHeight = 800;

    return [Math.round(baseWidth * scale), Math.round(baseHeight * scale)];
  }

  getContentMargins(): [number, number, number, number] {
    const spacing = this.spacingConfig;
    return [spacing.medium, spacing.medium, spacing.medium, spacing.medium];
  }

  getPanelWidth(): [number, number] {
    return [
      this.componentSize.panel.minWidth,
      this.componentSize.panel.maxWidth,
    ];
  }

  // Breakpoint helpers
  isSmall(): boolean {
    return this.config.breakpoint === "small";
  }

  isMedium(): boolean {
    return this.config.breakpoint === "medium";
  }

  isLarge(): boolean {
    return this.config.breakpoint === "large";
  }

  isXLarge(): boolean {
    return this.config.breakpoint === "xlarge";
  }

  isMobile(): boolean {
    return this.config.breakpoint === "small";
  }

  isTablet(): boolean {
    return this.config.breakpoint === "medium";
  }

  isDesktop(): boolean {
    return (
      this.config.breakpoint === "large" || this.config.breakpoint === "xlarge"
    );
  }
}

// Singleton instance
let responsiveManager: ResponsiveManager | null = null;

export function getResponsiveManager(): ResponsiveManager {
  if (!responsiveManager) {
    responsiveManager = new ResponsiveManager();
  }
  return responsiveManager;
}

// React hook for responsive design
export function useResponsive() {
  const [responsiveState, setResponsiveState] = React.useState(() => {
    const rm = getResponsiveManager();
    return {
      config: rm.getConfig(),
      fontConfig: rm.getFontConfig(),
      spacingConfig: rm.getSpacingConfig(),
      componentSize: rm.getComponentSize(),
    };
  });

  React.useEffect(() => {
    const handleResponsiveChange = (event: CustomEvent) => {
      setResponsiveState(event.detail);
    };

    if (typeof window !== "undefined") {
      window.addEventListener(
        "responsive-change",
        handleResponsiveChange as EventListener
      );
      return () => {
        window.removeEventListener(
          "responsive-change",
          handleResponsiveChange as EventListener
        );
      };
    }
  }, []);

  return responsiveState;
}

// Import React for the hook (avoid circular imports)
import React from "react";
