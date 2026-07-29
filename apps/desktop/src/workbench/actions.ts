import { ActionRegistry, type ActionDefinition } from "../actions/registry";
import { ShortcutResolver, type ShortcutBinding } from "../actions/shortcuts";
import { topLevelRoutes, type RouteId } from "./routes";

export interface WorkbenchActionContext {
  activeItem: string;
  inspectorOpen: boolean;
  openCommandPalette: () => void;
  openSection: (routeId: RouteId, item: string) => void;
  panelOpen: boolean;
  routeId: RouteId;
  setInspectorOpen: (open: boolean) => void;
  setPanelOpen: (open: boolean) => void;
  setSidebarOpen: (open: boolean) => void;
  sidebarOpen: boolean;
}

const navigationActions: ActionDefinition<WorkbenchActionContext>[] =
  topLevelRoutes.map((route) => ({
    category: "Navigation",
    execute: (context) => context.openSection(route.id, route.sidebarItems[0]),
    id: `navigation.${route.id}`,
    keywords: ["open", "switch", route.id],
    label: `Open ${route.label}`,
  }));

const sectionActions: ActionDefinition<WorkbenchActionContext>[] =
  topLevelRoutes.flatMap((route) =>
    route.sidebarItems.map((item) => ({
      category: route.label,
      execute: (context) => context.openSection(route.id, item),
      id: `section.${route.id}.${slug(item)}`,
      keywords: [route.label, item],
      label: `Open ${item}`,
    })),
  );

export const workbenchActionDefinitions: readonly ActionDefinition<WorkbenchActionContext>[] =
  Object.freeze([
    {
      category: "View",
      defaultShortcuts: ["Ctrl+Shift+P", "Meta+Shift+P"],
      evaluate: (context) =>
        context.sidebarOpen
          ? { available: false, reason: "Close the contextual sidebar first." }
          : { available: true },
      execute: (context) => context.openCommandPalette(),
      id: "palette.open",
      keywords: ["command", "search", "actions"],
      label: "Open command palette",
    },
    {
      category: "Context",
      evaluate: () => ({ available: false, reason: "No additional actions are available in this milestone." }),
      execute: () => undefined,
      id: "context.more",
      keywords: ["more", "context"],
      label: "Show more contextual actions",
    },
    {
      category: "View",
      execute: (context) => context.setSidebarOpen(!context.sidebarOpen),
      id: "view.sidebar.toggle",
      keywords: ["context", "navigation", "compact"],
      label: "Toggle contextual sidebar",
    },
    {
      category: "View",
      execute: (context) => context.setInspectorOpen(!context.inspectorOpen),
      id: "view.inspector.toggle",
      keywords: ["details", "context"],
      label: "Toggle inspector",
    },
    {
      category: "View",
      execute: (context) => context.setPanelOpen(!context.panelOpen),
      id: "view.operations.toggle",
      keywords: ["bottom", "operations", "problems"],
      label: "Toggle operations panel",
    },
    ...navigationActions,
    ...sectionActions,
  ]);

export const workbenchActionRegistry = new ActionRegistry(
  workbenchActionDefinitions,
);

const bindings: readonly ShortcutBinding[] = [
  { actionId: "palette.open", chord: "Ctrl+Shift+P", scope: "global" },
  { actionId: "palette.open", chord: "Meta+Shift+P", scope: "global" },
];

export const workbenchShortcutResolver = new ShortcutResolver(
  workbenchActionDefinitions,
  bindings,
);

export function sectionActionId(routeId: RouteId, item: string): string {
  return `section.${routeId}.${slug(item)}`;
}

function slug(value: string): string {
  return value.toLocaleLowerCase().replaceAll(/[^a-z0-9]+/gu, "-").replaceAll(/^-|-$/gu, "");
}
