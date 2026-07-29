# UI Architecture

## Goals

The workbench should feel calm for a new user and fast for a power user. It must be keyboard accessible, responsive with many panes, and adaptable to small and large screens without making infrastructure context disappear.

## Workbench model

The initial shell is organized around a workspace, not around a list of disconnected terminal windows:

```text
Application window
├── command/search layer
├── primary navigation (workspaces, hosts, snippets, settings)
├── contextual sidebar (workspace or session tools)
├── editor area (tabs and split-pane layout tree)
└── status and operation feedback
```

The terminal view is one editor surface. Future surfaces such as a file browser, log viewer, port-forward manager, and infrastructure overview use the same layout and operation feedback patterns.

## Component rules

- Components express visual behavior and user intent; they do not own infrastructure side effects.
- Reusable components must expose semantic states such as loading, unavailable, error, and destructive confirmation.
- Design tokens are consumed through semantic roles, not hard-coded colors or spacing values.
- A feature view must work with keyboard navigation and screen readers before it is considered complete.
- Modal dialogs are reserved for decisions that genuinely require interruption; use inline or panel feedback for routine status.
- Virtualize long lists and log views; never render unbounded session output as a normal DOM tree.

## Interaction contracts

Every action that can affect an external system should make these visible where applicable:

- target host and workspace;
- authenticated identity;
- command or operation;
- affected path, port, resource, or process;
- whether the action is read-only, reversible, or destructive;
- progress, cancellation, and final result.

## Responsive and platform behavior

The information architecture stays consistent across Windows, Linux, and macOS. Keyboard shortcuts, title-bar treatment, system menus, context menus, file pickers, and secure input may use platform-specific adapters. Platform-specific behavior must be documented and tested rather than hidden behind assumptions.

## Accessibility baseline

- full keyboard navigation and visible focus;
- semantic labels for icon-only controls;
- no color-only status indication;
- reduced-motion mode;
- minimum contrast targets aligned with WCAG 2.2 AA;
- sensible text scaling and terminal font controls;
- clear announcements for connection, transfer, and operation status.

## Visual quality gates

Before a UI feature is merged, it should have a design note or story, loading/error/empty states, keyboard behavior, light/dark or theme-token behavior, and a screenshot or reviewable test scenario when visual regression matters.
