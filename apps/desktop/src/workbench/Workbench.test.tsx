import "@testing-library/jest-dom/vitest";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

afterEach(cleanup);
import { SurfaceState } from "./SurfaceState";
import { Workbench } from "./Workbench";
import { topLevelRoutes } from "./routes";

describe("workbench navigation", () => {
  it("makes every top-level destination keyboard reachable", () => {
    render(<Workbench />);
    const primary = screen.getByRole("navigation", { name: "Primary" });
    for (const route of topLevelRoutes) {
      const destination = within(primary).getByRole("button", { name: route.label });
      destination.focus();
      fireEvent.keyDown(destination, { key: "Enter" });
      fireEvent.click(destination);
      if (route.id === "workspaces") expect(destination).toHaveFocus();
      else expect(screen.getByRole("heading", { name: route.sidebarItems[0] })).toHaveFocus();
      expect(destination).toHaveAttribute("aria-current", "page");
    }
  });

  it("uses arrow keys to move through the activity rail", () => {
    render(<Workbench />);
    const primary = screen.getByRole("navigation", { name: "Primary" });
    const workspaces = within(primary).getByRole("button", { name: "Workspaces" });
    workspaces.focus();
    fireEvent.keyDown(workspaces, { key: "ArrowDown" });
    expect(within(primary).getByRole("button", { name: "Hosts" })).toHaveFocus();
  });

  it("changes contextual sections without claiming future capability", () => {
    render(<Workbench />);
    fireEvent.click(screen.getByRole("button", { name: "Sessions" }));
    expect(screen.getByRole("heading", { name: "Sessions" })).toHaveFocus();
    expect(screen.getByText("Preview only")).toBeInTheDocument();
    expect(screen.getByText(/Local compositions of sessions/u)).toBeInTheDocument();
  });

  it("opens the command palette from the registered global shortcut", () => {
    render(<Workbench />);
    const trigger = screen.getByRole("button", { name: "Inspector" });
    trigger.focus();
    fireEvent.keyDown(window, { key: "P", ctrlKey: true, shiftKey: true });
    expect(screen.getByRole("textbox", { name: "Search actions and destinations" })).toHaveFocus();
    fireEvent.keyDown(screen.getByRole("dialog"), { key: "Escape" });
    expect(trigger).toHaveFocus();
  });

  it("opens and closes secondary regions with explicit focus restoration", () => {
    render(<Workbench />);
    const inspectorToggle = screen.getByRole("button", { name: "Inspector" });
    fireEvent.click(inspectorToggle);
    expect(inspectorToggle).toHaveAttribute("aria-expanded", "true");
    fireEvent.click(screen.getByRole("button", { name: "Close inspector" }));
    expect(inspectorToggle).toHaveFocus();

    const panelToggle = screen.getByRole("button", { name: "Open panel" });
    fireEvent.click(panelToggle);
    expect(screen.getByText("No operations are running.")).toBeInTheDocument();
  });
});

describe("workbench surface states", () => {
  it.each([
    ["empty", "Nothing here"],
    ["loading", "Loading"],
    ["error", "Could not load"],
    ["unavailable", "Unavailable"],
  ] as const)("renders the %s state with text semantics", (kind, title) => {
    render(
      <SurfaceState
        description="The state has an explicit explanation."
        kind={kind}
        title={title}
      />,
    );
    const state =
      kind === "error" ? screen.getByRole("alert") : screen.getByRole("status");
    expect(state).toHaveTextContent(title);
    expect(state).toHaveTextContent("explicit explanation");
  });
});
