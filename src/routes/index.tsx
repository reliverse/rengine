import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
  component: RengineHome,
});

function RengineHome() {
  // WelcomeScreen is now handled in the root route for full-screen rendering
  return null;
}
