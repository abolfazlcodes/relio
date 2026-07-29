import { describe, expect, it, vi } from "vitest";
import { ActionRegistry, type ActionDefinition } from "./registry";
import {
  ShortcutResolver,
  findShortcutConflicts,
  type ShortcutBinding,
} from "./shortcuts";

interface Context {
  allowed: boolean;
}

const definitions: readonly ActionDefinition<Context>[] = [
  {
    category: "Fixture",
    execute: vi.fn(),
    id: "fixture.available",
    keywords: ["alpha"],
    label: "Available action",
  },
  {
    category: "Fixture",
    evaluate: (context) =>
      context.allowed
        ? { available: true }
        : { available: false, reason: "A fixture capability is unavailable." },
    execute: vi.fn(),
    id: "fixture.conditional",
    label: "Conditional action",
  },
];

describe("action registry", () => {
  it("evaluates availability before dispatch and records recent use locally", async () => {
    const registry = new ActionRegistry(definitions);
    await expect(
      registry.dispatch("fixture.conditional", { allowed: false }),
    ).resolves.toBe(false);
    expect(registry.recentIds()).toHaveLength(0);
    await expect(
      registry.dispatch("fixture.available", { allowed: true }),
    ).resolves.toBe(true);
    expect(registry.recentIds()).toEqual(["fixture.available"]);
  });

  it("returns disabled reasons and never exceeds the bounded result page", () => {
    const many = Array.from({ length: 80 }, (_, index) => ({
      category: "Bounded",
      execute: () => undefined,
      id: `bounded.action-${index}`,
      label: `Bounded action ${index}`,
    }));
    const registry = new ActionRegistry([...definitions, ...many]);
    const results = registry.search("", { allowed: false });
    expect(results).toHaveLength(50);
    expect(
      registry.search("conditional", { allowed: false })[0],
    ).toMatchObject({
      available: false,
      reason: "A fixture capability is unavailable.",
    });
  });

  it("searches 1,000 bundled actions inside the 100 ms budget", () => {
    const actions = Array.from({ length: 1_000 }, (_, index) => ({
      category: "Performance",
      execute: () => undefined,
      id: `performance.action-${index}`,
      label: `Open performance destination ${index}`,
    }));
    const registry = new ActionRegistry(actions);
    const started = performance.now();
    expect(registry.search("destination 999", { allowed: true })[0]?.actionId)
      .toBe("performance.action-999");
    expect(performance.now() - started).toBeLessThan(100);
  });

  it("rejects duplicate, malformed, and excessive registrations", () => {
    expect(
      () => new ActionRegistry([{ ...definitions[0]!, id: "<remote-label>" }]),
    ).toThrow("action.id_invalid");
    expect(
      () => new ActionRegistry([definitions[0]!, definitions[0]!]),
    ).toThrow("action.id_duplicate");
    expect(
      () =>
        new ActionRegistry(
          Array.from({ length: 1_001 }, (_, index) => ({
            category: "Limit",
            execute: () => undefined,
            id: `limit.action-${index}`,
            label: "Bounded",
          })),
        ),
    ).toThrow("action.limit_exceeded");
  });
});

describe("shortcut resolver", () => {
  const actions = definitions;

  it("rejects unsafe printable globals and duplicate active chords", () => {
    expect(
      () =>
        new ShortcutResolver(actions, [
          { actionId: "fixture.available", chord: "x", scope: "global" },
        ]),
    ).toThrow("shortcut.chord_unsafe");
    const conflict: ShortcutBinding[] = [
      { actionId: "fixture.available", chord: "Ctrl+Shift+P", scope: "global" },
      { actionId: "fixture.conditional", chord: "Ctrl+Shift+P", scope: "global" },
    ];
    expect(findShortcutConflicts(conflict)[0]?.actionIds).toHaveLength(2);
    expect(() => new ShortcutResolver(actions, conflict)).toThrow(
      "shortcut.conflict",
    );
  });

  it("resolves only explicit application chords", () => {
    const resolver = new ShortcutResolver(actions, [
      { actionId: "fixture.available", chord: "Ctrl+Shift+P", scope: "global" },
    ]);
    expect(
      resolver.resolve(
        new KeyboardEvent("keydown", {
          ctrlKey: true,
          key: "P",
          shiftKey: true,
        }),
        "global",
      ),
    ).toBe("fixture.available");
    expect(
      resolver.resolve(new KeyboardEvent("keydown", { key: "x" }), "global"),
    ).toBeUndefined();
  });
});
