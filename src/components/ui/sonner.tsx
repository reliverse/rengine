import { useTheme } from "next-themes";
import { Toaster as Sonner, ToasterProps } from "sonner";

const Toaster = ({ ...props }: ToasterProps) => {
  const { theme = "system" } = useTheme();

  return (
    <Sonner
      className="toaster group"
      closeButton={true}
      duration={4000}
      expand={true}
      position="top-right"
      richColors={true}
      style={
        {
          "--normal-bg": "var(--popover)",
          "--normal-text": "var(--popover-foreground)",
          "--normal-border": "var(--border)",
          "--success-bg": "hsl(142 76% 36%)",
          "--success-text": "hsl(355 7% 97%)",
          "--error-bg": "hsl(0 84% 60%)",
          "--error-text": "hsl(355 7% 97%)",
          "--warning-bg": "hsl(38 92% 50%)",
          "--warning-text": "hsl(355 7% 97%)",
          "--info-bg": "hsl(199 89% 48%)",
          "--info-text": "hsl(355 7% 97%)",
        } as React.CSSProperties
      }
      theme={theme as ToasterProps["theme"]}
      {...props}
    />
  );
};

export { Toaster };
