import "@testing-library/jest-dom/vitest";
import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { App } from "./App";
import { FatalBootstrap } from "./FatalBootstrap";
import { isSupportedPlatform, UnsupportedPlatform } from "./UnsupportedPlatform";

describe("secure application shell", () => {
  it("renders a local-only empty state", () => {
    render(<App />);
    expect(screen.getByRole("heading", { name: "The secure application shell is ready." })).toBeInTheDocument();
    expect(screen.getByText("Local mode")).toBeInTheDocument();
  });
  it("renders a safe fatal state", () => {
    render(<FatalBootstrap />);
    expect(screen.getByRole("alert")).toHaveTextContent("No remote connection was attempted.");
  });
  it("rejects platforms outside the Tier 1 desktop set", () => {
    expect(isSupportedPlatform("Mozilla/5.0 (X11; Linux x86_64)")).toBe(true);
    expect(isSupportedPlatform("Mozilla/5.0 (Android 15; Linux)")).toBe(false);
    const view = render(<UnsupportedPlatform />);
    expect(view.container.querySelector("button")).toHaveTextContent("Exit Relio");
  });
});
