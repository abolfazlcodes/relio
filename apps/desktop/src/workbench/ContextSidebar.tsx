import type { RouteId } from "./routes";
import { routeById } from "./routes";

export interface ContextSidebarProps {
  activeItem: string;
  moreActionsReason: string;
  onMoreActions: () => void;
  onSelect: (item: string) => void;
  routeId: RouteId;
}

export function ContextSidebar({
  activeItem,
  moreActionsReason,
  onMoreActions,
  onSelect,
  routeId,
}: ContextSidebarProps) {
  const route = routeById(routeId);
  return (
    <aside aria-label={`${route.label} navigation`} className="context-sidebar">
      <header className="context-sidebar__header">
        <p>{route.label}</p>
        <button aria-disabled="true" aria-label={`More ${route.label} actions`} onClick={onMoreActions} title={moreActionsReason} type="button">•••</button>
      </header>
      <nav aria-label={`${route.label} sections`}>
        <ul className="context-sidebar__list">
          {route.sidebarItems.map((item) => (
            <li key={item}>
              <button
                aria-current={item === activeItem ? "page" : undefined}
                onClick={() => onSelect(item)}
                type="button"
              >
                {item}
              </button>
            </li>
          ))}
        </ul>
      </nav>
    </aside>
  );
}
