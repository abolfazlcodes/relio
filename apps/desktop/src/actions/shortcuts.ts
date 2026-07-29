import type { ActionDefinition } from "./registry";

export type ShortcutScope = "global" | "terminal-focused";

export interface ShortcutBinding {
  actionId: string;
  chord: string;
  scope: ShortcutScope;
}

export interface ShortcutConflict {
  actionIds: readonly string[];
  chord: string;
  reason: string;
  scope: ShortcutScope;
}

export class ShortcutResolver<TContext> {
  private readonly bindings = new Map<string, ShortcutBinding>();

  public constructor(
    actions: readonly ActionDefinition<TContext>[],
    bindings: readonly ShortcutBinding[],
  ) {
    const actionIds = new Set(actions.map((action) => action.id));
    for (const binding of bindings) {
      if (!actionIds.has(binding.actionId)) throw new Error("shortcut.action_unknown");
      const chord = normalizeChord(binding.chord);
      if (!isSafeChord(chord, binding.scope)) throw new Error("shortcut.chord_unsafe");
      const key = `${binding.scope}:${chord}`;
      if (this.bindings.has(key)) throw new Error("shortcut.conflict");
      this.bindings.set(key, { ...binding, chord });
    }
  }

  public resolve(event: KeyboardEvent, scope: ShortcutScope): string | undefined {
    if (event.isComposing || event.repeat) return undefined;
    const chord = eventToChord(event);
    return (
      this.bindings.get(`${scope}:${chord}`) ??
      this.bindings.get(`global:${chord}`)
    )?.actionId;
  }
}

export function findShortcutConflicts(
  bindings: readonly ShortcutBinding[],
): ShortcutConflict[] {
  const groups = new Map<string, ShortcutBinding[]>();
  for (const binding of bindings) {
    const chord = normalizeChord(binding.chord);
    const key = `${binding.scope}:${chord}`;
    groups.set(key, [...(groups.get(key) ?? []), { ...binding, chord }]);
  }
  return [...groups.values()]
    .filter((group) => group.length > 1)
    .map((group) => ({
      actionIds: group.map((binding) => binding.actionId),
      chord: group[0]?.chord ?? "",
      reason: "Two active actions use the same scope and chord.",
      scope: group[0]?.scope ?? "global",
    }));
}

function eventToChord(event: KeyboardEvent): string {
  const modifiers = [
    event.ctrlKey ? "Ctrl" : "",
    event.altKey ? "Alt" : "",
    event.shiftKey ? "Shift" : "",
    event.metaKey ? "Meta" : "",
  ].filter(Boolean);
  return normalizeChord([...modifiers, event.key].join("+"));
}

function normalizeChord(chord: string): string {
  const aliases: Record<string, string> = {
    cmd: "Meta",
    command: "Meta",
    control: "Ctrl",
    option: "Alt",
  };
  return chord
    .split("+")
    .map((part) => {
      const trimmed = part.trim();
      return aliases[trimmed.toLocaleLowerCase()] ?? (
        trimmed.length === 1 ? trimmed.toLocaleUpperCase() : trimmed
      );
    })
    .join("+");
}

function isSafeChord(chord: string, scope: ShortcutScope): boolean {
  const parts = chord.split("+");
  const key = parts.at(-1) ?? "";
  const hasModifier = parts.some((part) =>
    ["Ctrl", "Alt", "Shift", "Meta"].includes(part),
  );
  if (scope === "global" && key.length === 1 && !hasModifier) return false;
  return !(
    parts.length === 1 &&
    ["Escape", "Enter", "Space", " "].includes(key)
  );
}
