import type { LucideIcon } from "lucide-react";
import { Grid3X3, Monitor, Palette } from "lucide-react";
import { useTheme } from "./theme-provider";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Label } from "./ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";

interface SettingSectionProps {
  icon: LucideIcon;
  title: string;
  description: string;
  children: React.ReactNode;
}

function SettingSection({
  icon: Icon,
  title,
  description,
  children,
}: SettingSectionProps) {
  return (
    <Card>
      <CardHeader className="pb-3">
        <CardTitle className="flex items-center gap-2 text-lg">
          <Icon className="h-5 w-5" />
          {title}
        </CardTitle>
        <p className="text-muted-foreground text-sm">{description}</p>
      </CardHeader>
      <CardContent className="space-y-4">{children}</CardContent>
    </Card>
  );
}

export default function SettingsPanel() {
  const { theme, setTheme } = useTheme();

  return (
    <div className="space-y-6 p-6">
      <div className="mb-6">
        <h2 className="mb-2 font-bold text-3xl">Rengine Settings</h2>
        <p className="text-muted-foreground">Customize your experience</p>
      </div>

      <SettingSection
        description="Customize the look and feel"
        icon={Palette}
        title="Appearance"
      >
        <div className="space-y-4">
          <div className="space-y-2">
            <Label>Theme</Label>
            <Select
              onValueChange={(value) =>
                setTheme(value as "light" | "dark" | "system")
              }
              value={theme}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="light">Light</SelectItem>
                <SelectItem value="dark">Dark</SelectItem>
                <SelectItem value="system">System</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>
      </SettingSection>

      <SettingSection
        description="Configure the 3D viewport and rendering"
        icon={Monitor}
        title="3D Viewport"
      >
        <div className="space-y-4">
          <div className="text-muted-foreground text-sm">
            3D viewport settings coming soon...
          </div>
        </div>
      </SettingSection>

      <SettingSection
        description="Customize grid and visual guides"
        icon={Grid3X3}
        title="Grid & Guides"
      >
        <div className="space-y-4">
          <div className="text-muted-foreground text-sm">
            Grid and guide settings coming soon...
          </div>
        </div>
      </SettingSection>
    </div>
  );
}
