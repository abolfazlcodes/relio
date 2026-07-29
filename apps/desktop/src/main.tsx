import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App";
import { BootstrapBoundary } from "./BootstrapBoundary";
import { isSupportedPlatform, UnsupportedPlatform } from "./UnsupportedPlatform";
import "./ui/tokens.css";
import "./ui/components.css";
import "./actions/actions.css";
import "./workbench/workbench.css";
import "./styles.css";

const element = document.getElementById("root");
if (element === null) {
  document.body.textContent = "Relio could not initialize its bundled interface.";
} else {
  const root = createRoot(element);
  root.render(
    <StrictMode>
      <BootstrapBoundary>
        {isSupportedPlatform(navigator.userAgent) ? <App /> : <UnsupportedPlatform />}
      </BootstrapBoundary>
    </StrictMode>,
  );
}
