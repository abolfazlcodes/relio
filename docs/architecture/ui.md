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

The terminal view is one editor surface. The remote file browser, editor, log
viewer, and port-forward manager use the same layout and operation feedback
patterns.

The workbench renders only bundled core code. Remote pages, downloaded scripts,
and imported styles are not workbench surfaces.

## Component rules

- Components express visual behavior and user intent; they do not own infrastructure side effects.
- Reusable components must expose semantic states such as loading, unavailable, error, and destructive confirmation.
- Design tokens are consumed through semantic roles, not hard-coded colors or spacing values.
- A feature view must work with keyboard navigation and screen readers before it is considered complete.
- Modal dialogs are reserved for decisions that genuinely require interruption; use inline or panel feedback for routine status.
- Virtualize long lists and log views; never render unbounded session output as a normal DOM tree.
- Treat terminal text, remote filenames, Markdown, theme metadata, imported
  text, and diagnostics as untrusted; sanitize and encode at the
  renderer boundary.
- Do not turn a hyperlink or URI from remote or imported content into a privileged
  action. External navigation shows destination and requires an approved scheme.

## Interaction contracts

Every action that can affect an external system should make these visible where applicable:

- target host and workspace;
- authenticated identity;
- command or operation;
- affected path, port, resource, or process;
- whether the action is read-only, reversible, or destructive;
- progress, cancellation, and final result.

## Trusted safety chrome

Credential prompts, host-key review, update verification failures, broad port
binds, remote overwrite confirmation, and destructive operations use core-owned
components with a reserved visual identity.

- Untrusted terminal, remote, and imported content cannot invoke these
  components with an “approved” result.
- Themes may adapt contrast-safe tokens but cannot hide, resize below minimum,
  replace wording, or visually imitate the reserved trust indicator.
- The core binds the displayed target and operation ID to the action that will
  execute.
- Repeated or replayed confirmation responses are rejected.

## Responsive and platform behavior

The information architecture stays consistent across Windows, Linux, and macOS. Keyboard shortcuts, title-bar treatment, system menus, context menus, file pickers, and secure input may use platform-specific adapters. Platform-specific behavior must be documented and tested rather than hidden behind assumptions.

The support tiers and platform adapters are defined in
[platform support](platform-support.md). The UI exposes unavailable capabilities
with remediation instead of rendering controls that fail generically.

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

Security-sensitive surfaces also require spoofing, truncation, text-scaling,
localization-length, and hostile-input fixtures.
