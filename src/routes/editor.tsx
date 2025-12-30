import { createFileRoute } from "@tanstack/react-router";
import { RengineEditor } from "~/components/rengine-editor";

export const Route = createFileRoute("/editor")({
  component: EditorPage,
});

function EditorPage() {
  return <RengineEditor />;
}
