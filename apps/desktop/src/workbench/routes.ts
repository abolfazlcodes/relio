export const topLevelRoutes = [
  {
    id: "workspaces",
    label: "Workspaces",
    icon: "W",
    description: "Local compositions of sessions, hosts, files, and operations.",
    sidebarItems: ["Overview", "Sessions", "Hosts", "Remote files", "Port forwards", "Activity"],
  },
  {
    id: "hosts",
    label: "Hosts",
    icon: "H",
    description: "Reusable connection definitions. No connection is attempted from this view.",
    sidebarItems: ["All hosts", "Favorites", "Recently used", "Groups", "Environments", "Tags"],
  },
  {
    id: "library",
    label: "Library",
    icon: "L",
    description: "Local snippets, history, and recordings become available in later milestones.",
    sidebarItems: ["Snippets", "History", "Recordings"],
  },
  {
    id: "settings",
    label: "Settings",
    icon: "S",
    description: "Application configuration. Persistence is not available in this milestone.",
    sidebarItems: ["Appearance", "Terminal", "Keyboard", "Connections", "Credentials", "Privacy and data", "Diagnostics"],
  },
] as const;

export type RouteId = (typeof topLevelRoutes)[number]["id"];

export function isRouteId(value: string): value is RouteId {
  return topLevelRoutes.some((route) => route.id === value);
}

export function routeById(id: RouteId) {
  return topLevelRoutes.find((route) => route.id === id) ?? topLevelRoutes[0];
}
