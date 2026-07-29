import { useRef, type KeyboardEvent } from "react";
import { topLevelRoutes, type RouteId } from "./routes";

export interface ActivityRailProps {
  activeRoute: RouteId;
  onNavigate: (route: RouteId) => void;
  onToggleOperations: () => void;
}

export function ActivityRail({ activeRoute, onNavigate, onToggleOperations }: ActivityRailProps) {
  const refs = useRef(new Map<RouteId, HTMLButtonElement>());

  function handleKeyDown(event: KeyboardEvent, index: number): void {
    if (!["ArrowDown", "ArrowUp", "Home", "End"].includes(event.key)) return;
    event.preventDefault();
    let next = index;
    if (event.key === "Home") next = 0;
    if (event.key === "End") next = topLevelRoutes.length - 1;
    if (event.key === "ArrowDown") next = (index + 1) % topLevelRoutes.length;
    if (event.key === "ArrowUp") next = (index - 1 + topLevelRoutes.length) % topLevelRoutes.length;
    const route = topLevelRoutes[next];
    if (route) refs.current.get(route.id)?.focus();
  }

  return (
    <nav aria-label="Primary" className="activity-rail">
      <div className="activity-rail__brand" aria-label="Relio">R</div>
      <div className="activity-rail__destinations">
        {topLevelRoutes.map((route, index) => (
          <button
            aria-current={route.id === activeRoute ? "page" : undefined}
            aria-label={route.label}
            className="activity-rail__button"
            key={route.id}
            onClick={() => onNavigate(route.id)}
            onKeyDown={(event) => handleKeyDown(event, index)}
            ref={(element) => {
              if (element) refs.current.set(route.id, element);
              else refs.current.delete(route.id);
            }}
            title={route.label}
            type="button"
          >
            <span aria-hidden="true">{route.icon}</span>
          </button>
        ))}
      </div>
      <button
        aria-label="Open operations panel"
        onClick={onToggleOperations}
        className="activity-rail__button activity-rail__operations"
        title="Operations"
        type="button"
      >
        <span aria-hidden="true">0</span>
      </button>
    </nav>
  );
}
