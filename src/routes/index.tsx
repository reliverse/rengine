import { createFileRoute } from "@tanstack/react-router";
import { RengineEditor } from "~/components/rengine-editor";

export const Route = createFileRoute("/")({
  component: RengineHome,
});

function RengineHome() {
  return <RengineEditor />;
}
