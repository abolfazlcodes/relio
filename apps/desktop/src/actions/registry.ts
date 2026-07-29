export interface ActionAvailability {
  available: boolean;
  reason?: string;
}

export interface ActionDefinition<TContext> {
  category: string;
  defaultShortcuts?: readonly string[];
  evaluate?: (context: TContext) => ActionAvailability;
  execute: (context: TContext) => void | Promise<void>;
  id: string;
  keywords?: readonly string[];
  label: string;
}

export interface ActionResult {
  actionId: string;
  available: boolean;
  category: string;
  label: string;
  reason?: string;
  score: number;
  shortcut?: string;
}

const MAX_ACTIONS = 1_000;
const MAX_QUERY_LENGTH = 256;
const MAX_RESULTS = 50;
const MAX_RECENT = 20;

export class ActionRegistry<TContext> {
  private readonly actions: ReadonlyMap<string, ActionDefinition<TContext>>;
  private readonly recent: string[] = [];

  public constructor(definitions: readonly ActionDefinition<TContext>[]) {
    if (definitions.length > MAX_ACTIONS) throw new Error("action.limit_exceeded");
    const actions = new Map<string, ActionDefinition<TContext>>();
    for (const definition of definitions) {
      if (!/^[a-z][a-z0-9_.-]{2,127}$/u.test(definition.id)) {
        throw new Error("action.id_invalid");
      }
      if (actions.has(definition.id)) throw new Error("action.id_duplicate");
      actions.set(definition.id, Object.freeze({ ...definition }));
    }
    this.actions = actions;
  }

  public get(id: string): ActionDefinition<TContext> | undefined {
    return this.actions.get(id);
  }

  public availability(id: string, context: TContext): ActionAvailability {
    const action = this.actions.get(id);
    if (!action) return { available: false, reason: "Action is not registered." };
    return action.evaluate?.(context) ?? { available: true };
  }

  public async dispatch(id: string, context: TContext): Promise<boolean> {
    const action = this.actions.get(id);
    if (!action || !this.availability(id, context).available) return false;
    await action.execute(context);
    const prior = this.recent.indexOf(id);
    if (prior >= 0) this.recent.splice(prior, 1);
    this.recent.unshift(id);
    this.recent.splice(MAX_RECENT);
    return true;
  }

  public search(query: string, context: TContext): ActionResult[] {
    const normalized = normalizeQuery(query.slice(0, MAX_QUERY_LENGTH));
    const recentRank = new Map(this.recent.map((id, index) => [id, index]));
    return [...this.actions.values()]
      .map((action) => {
        const availability = this.availability(action.id, context);
        const haystack = normalizeQuery(
          `${action.label} ${action.category} ${(action.keywords ?? []).join(" ")}`,
        );
        const score = normalized ? fuzzyScore(normalized, haystack) : 1;
        const recency = recentRank.get(action.id);
        return {
          actionId: action.id,
          available: availability.available,
          category: action.category,
          label: action.label,
          ...(availability.reason ? { reason: availability.reason } : {}),
          score: score < 0 ? score : score + (recency === undefined ? 0 : 100 - recency),
          ...(action.defaultShortcuts?.[0]
            ? { shortcut: action.defaultShortcuts[0] }
            : {}),
        };
      })
      .filter((result) => result.score >= 0)
      .sort((left, right) => right.score - left.score || left.label.localeCompare(right.label))
      .slice(0, MAX_RESULTS);
  }

  public recentIds(): readonly string[] {
    return this.recent;
  }
}

function normalizeQuery(value: string): string {
  return value.normalize("NFKC").toLocaleLowerCase().trim();
}

function fuzzyScore(needle: string, haystack: string): number {
  if (haystack.includes(needle)) return 200 - haystack.indexOf(needle);
  let cursor = 0;
  let score = 0;
  for (const character of needle) {
    const found = haystack.indexOf(character, cursor);
    if (found < 0) return -1;
    score += Math.max(1, 20 - (found - cursor));
    cursor = found + 1;
  }
  return score;
}
