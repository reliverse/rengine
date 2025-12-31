import type { LucideIcon } from "lucide-react";
import {
  Grid3X3,
  Monitor,
  Palette,
  Settings as SettingsIcon,
} from "lucide-react";
import { useSettingsStore } from "~/stores/settings-store";
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
  const { precision, setPrecision } = useSettingsStore();

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

      <SettingSection
        description="Configure numeric precision for values and exports"
        icon={SettingsIcon}
        title="Precision"
      >
        <div className="space-y-4">
          <div className="space-y-2">
            <Label>Decimal Places</Label>
            <Select
              onValueChange={(value) =>
                value && setPrecision(Number.parseInt(value, 10))
              }
              value={precision.toString()}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="1">1 decimal place</SelectItem>
                <SelectItem value="2">2 decimal places</SelectItem>
                <SelectItem value="3">3 decimal places</SelectItem>
                <SelectItem value="4">4 decimal places</SelectItem>
                <SelectItem value="5">5 decimal places</SelectItem>
                <SelectItem value="6">6 decimal places</SelectItem>
              </SelectContent>
            </Select>
            <p className="text-muted-foreground text-sm">
              Controls precision for position, rotation, scale values and
              exports.
            </p>
          </div>
        </div>
      </SettingSection>
    </div>
  );
}
