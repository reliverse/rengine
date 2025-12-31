import { createFileRoute } from "@tanstack/react-router";
import { WelcomeScreen } from "~/components/welcome-screen";

export const Route = createFileRoute("/welcome")({
  component: WelcomePage,
});

function WelcomePage() {
  return <WelcomeScreen />;
}
