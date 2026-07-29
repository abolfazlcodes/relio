import { lazy, Suspense, useEffect, useRef, useState } from "react";
import { CommandPalette } from "../actions/CommandPalette";
import type { WorkbenchActionContext } from "./actions";
import { sectionActionId, workbenchActionRegistry, workbenchShortcutResolver } from "./actions";
import { ActivityRail } from "./ActivityRail";
import { ContextSidebar } from "./ContextSidebar";
import { routeById, type RouteId } from "./routes";
import { SurfaceState } from "./SurfaceState";

const TerminalPane = lazy(async () => {
  const module = await import("../terminal/TerminalPane");
  return { default: module.TerminalPane };
});

export function Workbench() {
  const [routeId, setRouteId] = useState<RouteId>("workspaces");
  const route = routeById(routeId);
  const [activeItem, setActiveItem] = useState<string>(route.sidebarItems[0]);
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const [inspectorOpen, setInspectorOpen] = useState(false);
  const [panelOpen, setPanelOpen] = useState(false);
  const [paletteOpen, setPaletteOpen] = useState(false);
  const headingRef = useRef<HTMLHeadingElement>(null);
  const sidebarToggleRef = useRef<HTMLButtonElement>(null);
  const inspectorToggleRef = useRef<HTMLButtonElement>(null);

  const actionContext: WorkbenchActionContext = {
    activeItem,
    inspectorOpen,
    openCommandPalette: () => setPaletteOpen(true),
    openSection: (nextRoute, item) => { setRouteId(nextRoute); setActiveItem(item); },
    panelOpen,
    routeId,
    setInspectorOpen,
    setPanelOpen,
    setSidebarOpen,
    sidebarOpen,
  };

  useEffect(() => {
    setSidebarOpen(false);
    headingRef.current?.focus();
  }, [routeId, activeItem]);

  useEffect(() => {
    const listener = (event: KeyboardEvent) => {
      const actionId = workbenchShortcutResolver.resolve(event, "global");
      if (!actionId) return;
      event.preventDefault();
      void workbenchActionRegistry.dispatch(actionId, actionContext);
    };
    window.addEventListener("keydown", listener);
    return () => window.removeEventListener("keydown", listener);
  }, [actionContext]);

  return (
    <div className="workbench">
      <a className="skip-link" href="#workbench-content">Skip to active view</a>
      <ActivityRail activeRoute={routeId} onNavigate={(next) => { void workbenchActionRegistry.dispatch("navigation." + next, actionContext); }} onToggleOperations={() => { void workbenchActionRegistry.dispatch("view.operations.toggle", actionContext); }} />
      <header className="workbench-topbar">
        <button
          aria-controls="context-sidebar"
          aria-expanded={sidebarOpen}
          aria-label="Toggle contextual sidebar"
          className="compact-only"
          onClick={() => { void workbenchActionRegistry.dispatch("view.sidebar.toggle", actionContext); }}
          ref={sidebarToggleRef}
          type="button"
        >
          ☰
        </button>
        <div className="workbench-context">
          <strong>Relio</strong>
          <span aria-label="Local environment">◇ Local</span>
        </div>
        <div className="workbench-topbar__actions">
          <button
            aria-controls="context-inspector"
            aria-expanded={inspectorOpen}
            onClick={() => { void workbenchActionRegistry.dispatch("view.inspector.toggle", actionContext); }}
            ref={inspectorToggleRef}
            type="button"
          >
            Inspector
          </button>
        </div>
      </header>
      <div
        className="sidebar-slot"
        data-open={sidebarOpen || undefined}
        id="context-sidebar"
        onKeyDown={(event) => {
          if (event.key === "Escape" && sidebarOpen) {
            void workbenchActionRegistry.dispatch("view.sidebar.toggle", actionContext);
            sidebarToggleRef.current?.focus();
          }
        }}
      >
        <ContextSidebar
          activeItem={activeItem}
          moreActionsReason={workbenchActionRegistry.availability("context.more", actionContext).reason ?? "Unavailable"}
          onMoreActions={() => { void workbenchActionRegistry.dispatch("context.more", actionContext); }}
          onSelect={(item) => { void workbenchActionRegistry.dispatch(sectionActionId(routeId, item), actionContext); }}
          routeId={routeId}
        />
      </div>
      <main className="editor-region" id="workbench-content">
        <div aria-label="Open views" className="editor-tabs" role="tablist">
          <button aria-selected="true" role="tab" type="button">
            {activeItem}
          </button>
        </div>
        <section
          aria-labelledby="active-view-title"
          className="editor-surface"
          role="tabpanel"
        >
          <div className="editor-surface__heading">
            <div>
              <p className="eyebrow">{route.label}</p>
              <h1 id="active-view-title" ref={headingRef} tabIndex={-1}>
                {activeItem}
              </h1>
            </div>
            <span className="capability-label">{routeId === "workspaces" && activeItem === "Sessions" ? "Local runtime" : "Preview only"}</span>
          </div>
          {routeId === "workspaces" && activeItem === "Sessions" ? (
            <Suspense fallback={<SurfaceState description="Loading the bundled terminal renderer." kind="loading" title="Loading terminal" />}>
              <TerminalPane />
            </Suspense>
          ) : (
            <SurfaceState
              description={route.description}
              kind="empty"
              title={`No ${activeItem.toLocaleLowerCase()} yet`}
            />
          )}
        </section>
      </main>
      <aside
        aria-label="Context inspector"
        className="context-inspector"
        data-open={inspectorOpen || undefined}
        id="context-inspector"
        onKeyDown={(event) => {
          if (event.key === "Escape" && inspectorOpen) {
            void workbenchActionRegistry.dispatch("view.inspector.toggle", actionContext);
            inspectorToggleRef.current?.focus();
          }
        }}
      >
        <header>
          <h2>Inspector</h2>
          <button
            aria-label="Close inspector"
            onClick={() => {
              void workbenchActionRegistry.dispatch("view.inspector.toggle", actionContext);
              inspectorToggleRef.current?.focus();
            }}
            type="button"
          >
            ×
          </button>
        </header>
        <p>No item is selected. Infrastructure details will appear here only after their owning milestone.</p>
      </aside>
      <section
        aria-label="Bottom panel"
        className="bottom-panel"
        data-open={panelOpen || undefined}
      >
        <header>
          <h2>Operations</h2>
          <button
            aria-expanded={panelOpen}
            onClick={() => { void workbenchActionRegistry.dispatch("view.operations.toggle", actionContext); }}
            type="button"
          >
            {panelOpen ? "Collapse panel" : "Open panel"}
          </button>
        </header>
        {panelOpen ? <p>No operations are running.</p> : null}
      </section>
      <CommandPalette context={actionContext} onClose={() => setPaletteOpen(false)} open={paletteOpen} registry={workbenchActionRegistry} />
      <footer className="workbench-statusbar" aria-label="Application status">
        <span>Local mode</span>
        <span>No profile open</span>
        <span aria-live="polite">Ready</span>
      </footer>
    </div>
  );
}
