import "@testing-library/jest-dom/vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { CommandPalette } from "./CommandPalette";
import { ActionRegistry } from "./registry";

afterEach(cleanup);

describe("command palette", () => {
  it("focuses search, explains disabled actions, dispatches, and restores focus", async () => {
    const execute = vi.fn();
    const registry = new ActionRegistry([
      {
        category: "Navigation",
        execute,
        id: "navigation.enabled",
        label: "Open enabled view",
      },
      {
        category: "Navigation",
        evaluate: () => ({
          available: false,
          reason: "This view is not available.",
        }),
        execute: vi.fn(),
        id: "navigation.disabled",
        label: "Open disabled view",
      },
    ]);
    const onClose = vi.fn();
    const trigger = document.createElement("button");
    document.body.append(trigger);
    trigger.focus();
    const view = render(
      <CommandPalette
        context={{}}
        onClose={onClose}
        open
        registry={registry}
      />,
    );

    const search = screen.getByRole("textbox", {
      name: "Search actions and destinations",
    });
    expect(search).toHaveFocus();
    fireEvent.change(search, { target: { value: "disabled" } });
    expect(screen.getByText("This view is not available.")).toBeInTheDocument();
    fireEvent.keyDown(search, { key: "Enter" });
    expect(onClose).not.toHaveBeenCalled();

    fireEvent.change(search, { target: { value: "enabled" } });
    fireEvent.keyDown(search, { key: "Enter" });
    await waitFor(() => {
      expect(execute).toHaveBeenCalledOnce();
      expect(onClose).toHaveBeenCalledOnce();
    });

    view.rerender(
      <CommandPalette
        context={{}}
        onClose={onClose}
        open={false}
        registry={registry}
      />,
    );
    expect(trigger).toHaveFocus();
    trigger.remove();
  });

  it("renders markup-shaped labels as inert text", () => {
    const registry = new ActionRegistry([
      {
        category: "Fixture",
        execute: () => undefined,
        id: "fixture.hostile",
        label: "<img src=x onerror=alert(1)>",
      },
    ]);
    const { container } = render(
      <CommandPalette
        context={{}}
        onClose={() => undefined}
        open
        registry={registry}
      />,
    );
    expect(container.querySelector("img")).not.toBeInTheDocument();
    expect(screen.getByText("<img src=x onerror=alert(1)>")).toBeInTheDocument();
  });
});
