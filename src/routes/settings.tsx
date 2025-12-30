import { createFileRoute, useNavigate } from "@tanstack/react-router";
import SettingsPanel from "~/components/settings-panel";
import { Button } from "~/components/ui/button";

export const Route = createFileRoute("/settings")({
  component: SettingsPage,
  beforeLoad: () => ({
    meta: {
      title: "Settings | rengine",
    },
  }),
});

function SettingsPage() {
  const navigate = useNavigate();

  return (
    <div className="container mx-auto h-full max-h-screen overflow-y-auto px-4 py-8">
      <div className="mb-6">
        <Button
          className="mb-4 px-2 text-muted-foreground hover:text-foreground"
          onClick={() => navigate({ to: "/" })}
          variant="ghost"
        >
          ← Back
        </Button>
      </div>
      <SettingsPanel />
    </div>
  );
}
