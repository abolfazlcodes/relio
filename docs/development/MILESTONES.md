# Relio v1 Master Development Tracker

This document is the single source of truth for Relio v1 implementation
progress. The [roadmap](../roadmap.md) describes strategic phases; this tracker
defines the independently buildable units of work. Architecture, security, UX,
and maintenance documents remain authoritative for behavior and constraints.

## Overall Progress

**9 / 39 Milestones Complete**

**Progress: 23%**

## How to use this tracker

- Complete milestones in numerical order. A milestone’s Dependencies section lists
  its direct technical prerequisites; all lower-numbered milestones must also be
  complete before work starts.
- Exactly one status checkbox must be selected for each milestone.
- A milestone is complete only when every acceptance criterion and checklist
  item is complete and verification evidence is linked from its issue or pull
  request.
- “Blocked” must name the blocking decision, owner, and next action in the
  milestone’s tracking issue; do not add ad hoc notes to this document.
- Checklist items describe outcomes, not a prescribed implementation.
- Security, accessibility, documentation, platform behavior, cancellation,
  errors, and tests are part of each milestone—not deferred cleanup.
- Updating counts and statuses is part of the milestone pull request.

## Status legend

- **Not Started** — no implementation work is active.
- **In Progress** — implementation is active in one pull request.
- **Blocked** — progress cannot continue without a documented dependency.
- **Complete** — all acceptance criteria and checks have passed.

## Milestone Index

1. [Project Governance and Release Prerequisites](#milestone-01--project-governance-and-release-prerequisites)
2. [Repository and Toolchain Bootstrap](#milestone-02--repository-and-toolchain-bootstrap)
3. [Secure Desktop Application Shell](#milestone-03--secure-desktop-application-shell)
4. [Typed IPC, Errors, and Operation Foundation](#milestone-04--typed-ipc-errors-and-operation-foundation)
5. [Design System and Accessibility Baseline](#milestone-05--design-system-and-accessibility-baseline)
6. [Workbench Navigation and Application Layout](#milestone-06--workbench-navigation-and-application-layout)
7. [Action Registry and Command Palette](#milestone-07--action-registry-and-command-palette)
8. [Window, Startup, and Shutdown Lifecycle](#milestone-08--window-startup-and-shutdown-lifecycle)
9. [Local PTY Runtime](#milestone-09--local-pty-runtime)
10. [Terminal Rendering](#milestone-10--terminal-rendering)
11. [Session Manager](#milestone-11--session-manager)
12. [Terminal Tabs](#milestone-12--terminal-tabs)
13. [Split Panes and Layout Model](#milestone-13--split-panes-and-layout-model)
14. [Encrypted Profile and Persistence](#milestone-14--encrypted-profile-and-persistence)
15. [Settings Engine and Settings UI](#milestone-15--settings-engine-and-settings-ui)
16. [Theme Engine and Theme Editor](#milestone-16--theme-engine-and-theme-editor)
17. [Workspace Manager and Durable Layout Restore](#milestone-17--workspace-manager-and-durable-layout-restore)
18. [Host Manager](#milestone-18--host-manager)
19. [Credential Manager and Secret Service](#milestone-19--credential-manager-and-secret-service)
20. [OpenSSH Capability and Configuration Engine](#milestone-20--openssh-capability-and-configuration-engine)
21. [Host Identity and SSH Authentication](#milestone-21--host-identity-and-ssh-authentication)
22. [SSH Connection and Remote Session Manager](#milestone-22--ssh-connection-and-remote-session-manager)
23. [Durable Session Restore](#milestone-23--durable-session-restore)
24. [Bounded SFTP Transport](#milestone-24--bounded-sftp-transport)
25. [File Transfer Manager](#milestone-25--file-transfer-manager)
26. [Remote File Browser](#milestone-26--remote-file-browser)
27. [Remote Text Editor](#milestone-27--remote-text-editor)
28. [Port Forwarding Manager](#milestone-28--port-forwarding-manager)
29. [Command Snippets](#milestone-29--command-snippets)
30. [Opt-In Command History](#milestone-30--opt-in-command-history)
31. [Search Everywhere](#milestone-31--search-everywhere)
32. [Structured Logging, Diagnostics, and Log Viewer](#milestone-32--structured-logging-diagnostics-and-log-viewer)
33. [Session Recording](#milestone-33--session-recording)
34. [Cross-Platform Compatibility Hardening](#milestone-34--cross-platform-compatibility-hardening)
35. [Accessibility Completion](#milestone-35--accessibility-completion)
36. [Performance and Resource Hardening](#milestone-36--performance-and-resource-hardening)
37. [Native Packaging](#milestone-37--native-packaging)
38. [Secure Update and Artifact Promotion](#milestone-38--secure-update-and-artifact-promotion)
39. [Stable v1 Release Readiness](#milestone-39--stable-v1-release-readiness)

---

# Milestone 01 — Project Governance and Release Prerequisites

### Status

- [ ] Not Started
- [ ] In Progress
- [ ] Blocked
- [x] Complete

### Goal

Resolve decisions that must not be left to incidental implementation work.

### Description

Establishes the legal, ownership, platform, dependency-review, and security
reporting foundation required before implementation dependencies are accepted.

### Deliverables

- Selected project license and attribution policy.
- Named area owners and review-enforced ownership map.
- Named private security contact and incident owner.
- Provisional Tier 1 reference machines and supported build targets.
- Approved process for dependency adoption and ADR ownership.

### Dependencies

- None.

### Estimated Complexity

**Medium**

### Acceptance Criteria

- License and ownership decisions are published and internally consistent.
- Every critical area has a primary owner; release/security backup requirements
  are tracked before stable release.
- The security reporting path is private, usable, and documented.
- Tier 1 targets and performance reference systems have accountable owners.

### Security Notes

Do not create signing keys or store release secrets in the repository. Role
assignment follows least privilege and separation of duties.

### Testing Requirements

- Validate repository policy links and ownership coverage.
- Exercise the private vulnerability-reporting path with a synthetic report.
- Review governance decisions against release and security blockers.

### Checklist

- [x] Select and add the project license.
- [x] Publish attribution and third-party notice policy.
- [x] Create the review-enforced ownership map.
- [x] Assign security and incident ownership.
- [x] Record provisional Tier 1 machines and maintainers.
- [x] Confirm dependency and ADR approval workflow.
- [x] Update security and contribution entry points.
- [x] Record verification evidence.

---

# Milestone 02 — Repository and Toolchain Bootstrap

### Status

- [ ] Not Started
- [ ] In Progress
- [ ] Blocked
- [x] Complete

### Goal

Create a reproducible, contributor-friendly Rust, React, and Tauri workspace.

### Description

Introduces the minimum project structure, pinned toolchains, formatting,
linting, generated-contract location, and unprivileged CI needed for all later
work.

### Deliverables

- Cargo and pnpm workspaces with locked dependencies.
- Pinned Rust, Node.js, pnpm, Tauri CLI, and target configuration.
- Desktop app directories matching the repository architecture.
- Formatting, linting, type-checking, unit-test, license, advisory, and secret
  scanning checks.
- Contributor build and troubleshooting instructions.

### Dependencies

- Milestone 01.

### Estimated Complexity

**Medium**

### Acceptance Criteria

- A clean checkout installs from reviewed lockfiles and passes all checks.
- CI runs with read-only default permissions and no signing credentials.
- Development builds use identifiers and data paths distinct from stable builds.
- Generated files identify their source and freshness check.

### Security Notes

Review dependency scripts, native code, default features, Tauri plugins, and CI
actions. Pin third-party actions to reviewed commit hashes.

### Testing Requirements

- Clean Windows, macOS, and Linux bootstrap smoke tests.
- Lockfile reproducibility and stale-generated-file checks.
- Negative secret-scanning fixture.

### Checklist

- [x] Create Cargo workspace.
- [x] Create pnpm workspace.
- [x] Pin supported toolchains and package manager.
- [x] Add reviewed lockfiles.
- [x] Add formatting, linting, and strict type checks.
- [x] Add unit-test and contract-generation commands.
- [x] Add dependency, license, and secret scans.
- [x] Add least-privilege CI.
- [x] Document clean-checkout setup.
- [x] Verify all Tier 1 bootstrap jobs.

---

# Milestone 03 — Secure Desktop Application Shell

### Status

- [ ] Not Started
- [ ] In Progress
- [ ] Blocked
- [x] Complete

### Goal

Produce a launchable Relio desktop window with a minimal trusted surface.

### Description

Creates the Tauri composition root and bundled React shell without privileged
product capabilities, remote content, persistence, or network activity.

### Deliverables

- Primary desktop window and React application root.
- Bundled assets with restrictive content security policy.
- Explicit empty Tauri capability configuration.
- Loading, fatal error, and unsupported-platform surfaces.
- Local development and packaged smoke launch.

### Dependencies

- Milestone 02.

### Estimated Complexity

**Medium**

### Acceptance Criteria

- The packaged app launches and displays a usable empty shell on Tier 1 targets.
- Startup makes no network request and loads no remote code or assets.
- The webview has no generic shell, filesystem, network, database, or updater
  authority.
- Fatal bootstrap failures are visible and permit safe exit.

### Security Notes

Treat the webview as less trusted. Enable no convenience Tauri plugin or asset
scope without a reviewed requirement.

### Testing Requirements

- Packaged launch smoke tests on Tier 1 targets.
- CSP and remote-origin denial tests.
- Capability allow/deny snapshot tests.
- Startup network-observation test.

### Checklist

- [x] Create Tauri composition root.
- [x] Create React application root.
- [x] Bundle all required assets.
- [x] Configure restrictive CSP.
- [x] Declare explicit window capabilities.
- [x] Add bootstrap and fatal-error UI.
- [x] Add packaged launch smoke test.
- [x] Verify zero startup network activity.

---

# Milestone 04 — Typed IPC, Errors, and Operation Foundation

### Status

- [ ] Not Started
- [ ] In Progress
- [ ] Blocked
- [x] Complete

### Goal

Establish the only supported communication boundary between the webview and
Rust core.

### Description

Implements generated contracts, request/response envelopes, typed errors,
operation IDs, ordered metadata events, cancellation, limits, and diagnostic
correlation without product-specific privileged commands.

### Deliverables

- Rust-owned schema and generated TypeScript bindings.
- Typed command/query client.
- Error taxonomy and safe frontend error model.
- Operation registry, cancellation, idempotency, and event sequencing.
- Bounded stream-broker interface with a test transport.

### Dependencies

- Milestone 03.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- Generated bindings cannot drift from Rust contracts.
- Unknown versions, malformed messages, stale revisions, oversized payloads,
  replayed decisions, and out-of-state requests fail predictably.
- Every test operation reaches exactly one terminal outcome.
- Raw infrastructure errors and secret fields cannot cross into frontend DTOs.

### Security Notes

Expose intent-specific commands only. Enforce payload limits, per-window
capabilities, confirmation nonces, and diagnostic redaction in the core.

### Testing Requirements

- Rust/TypeScript contract compatibility tests.
- Fuzz/property tests for envelopes and event parsing.
- Capability denial, cancellation race, idempotency, event-gap, and size-limit
  tests.

### Checklist

- [x] Define versioned common envelopes.
- [x] Generate TypeScript bindings from Rust.
- [x] Add typed frontend client.
- [x] Add stable error codes and safe messages.
- [x] Add operation registry and cancellation.
- [x] Add idempotency handling.
- [x] Add ordered event subscriptions and gap recovery.
- [x] Add bounded stream-broker contract.
- [x] Pass malformed and hostile-input tests.

---

# Milestone 05 — Design System and Accessibility Baseline

### Status

- [ ] Not Started
- [ ] In Progress
- [ ] Blocked
- [x] Complete

### Goal

Create the reusable visual and interaction foundation for every Relio surface.

### Description

Implements semantic tokens, typography, spacing, controls, focus behavior,
overlays, status patterns, safety chrome, and accessibility test conventions
using the bundled default theme only.

### Deliverables

- Semantic token baseline and default light/dark appearances.
- Core buttons, inputs, lists, tabs, dialogs, notifications, and status
  components.
- Reserved trusted-confirmation surface.
- Reduced-motion, text-scaling, high-contrast, and keyboard foundations.
- Component documentation and visual fixtures.

### Dependencies

- Milestone 03.

### Estimated Complexity

**Medium**

### Acceptance Criteria

- Core components meet WCAG 2.2 AA requirements in supported modes.
- Focus is visible and deterministic; status never relies on color alone.
- Untrusted content cannot render as trusted confirmation chrome.
- Components work at documented minimum window size and text scaling.

### Security Notes

Safety chrome is structurally reserved and cannot be hidden or restyled by
untrusted content or future theme data.

### Testing Requirements

- Component unit, keyboard, semantic, and accessibility tests.
- Light/dark/high-contrast visual regression.
- Long, hostile, bidirectional, and non-Latin text fixtures.

### Checklist

- [x] Define semantic token baseline.
- [x] Implement core controls and states.
- [x] Implement focus and keyboard conventions.
- [x] Implement trusted confirmation component.
- [x] Add reduced-motion and text-scaling behavior.
- [x] Add component accessibility tests.
- [x] Add visual regression fixtures.
- [x] Document component usage boundaries.

---

# Milestone 06 — Workbench Navigation and Application Layout

### Status

- [ ] Not Started
- [ ] In Progress
- [ ] Blocked
- [x] Complete

### Goal

Deliver a navigable, responsive Relio workbench with useful empty states.

### Description

Adds the activity rail, contextual sidebar, editor area, bottom panel, status
bar, routing, focus restoration, responsive collapse behavior, and static
placeholder views for in-scope features.

### Deliverables

- Workbench layout and route model.
- Activity rail and contextual sidebar.
- Editor tab region, bottom panel, inspector, and status bar.
- Empty, unavailable, loading, and error states.
- Narrow-window and keyboard navigation behavior.

### Dependencies

- Milestones 04–05.

### Estimated Complexity

**Medium**

### Acceptance Criteria

- Every v1 top-level destination is reachable without a mouse.
- Route changes preserve or intentionally restore focus.
- Narrow windows collapse secondary regions without hiding target/safety
  context.
- Placeholder views make no unsupported capability claim.

### Security Notes

Terminal or remote content never controls application routing, navigation
labels, or trusted status.

### Testing Requirements

- Route, focus, keyboard, responsive-layout, and accessibility tests.
- Visual regressions at minimum, standard, and large window sizes.

### Checklist

- [x] Implement route model.
- [x] Implement activity rail.
- [x] Implement contextual sidebar.
- [x] Implement editor and tab region.
- [x] Implement bottom panel and status bar.
- [x] Implement inspector behavior.
- [x] Add empty/loading/error/unavailable states.
- [x] Verify keyboard and narrow-window navigation.

---

# Milestone 07 — Action Registry and Command Palette

### Status

- [ ] Not Started
- [ ] In Progress
- [ ] Blocked
- [x] Complete

### Goal

Provide one discoverable action model shared by menus, buttons, shortcuts, and
the command palette.

### Description

Introduces named actions, availability reasons, shortcut dispatch, fuzzy
palette search, recent actions, and keyboard conflict handling for capabilities
implemented so far.

### Deliverables

- Typed action registry and context evaluation.
- Command palette with bounded local search.
- Shortcut resolver and conflict diagnostics.
- Menu/button adapters using the same action definitions.
- Action telemetry-free recent-use state in memory.

### Dependencies

- Milestones 04 and 06.

### Estimated Complexity

**Medium**

### Acceptance Criteria

- All existing user actions resolve through one registry.
- Disabled actions explain why they are unavailable.
- Palette opens within the documented performance budget.
- Application shortcuts do not synthesize terminal input.

### Security Notes

Actions express frontend intent only; the core reauthorizes every privileged
operation. Untrusted labels cannot register actions.

### Testing Requirements

- Action availability and dispatch unit tests.
- Palette performance and keyboard E2E tests.
- Shortcut conflict and untrusted-label tests.

### Checklist

- [x] Define action contract.
- [x] Add context and availability evaluation.
- [x] Implement command palette.
- [x] Implement shortcut resolver.
- [x] Connect menus and controls to actions.
- [x] Add disabled-reason UI.
- [x] Verify bounded search and keyboard behavior.

---

# Milestone 08 — Window, Startup, and Shutdown Lifecycle

### Status

- [ ] Not Started
- [ ] In Progress
- [ ] Blocked
- [x] Complete

### Goal

Make desktop lifecycle behavior deterministic and recoverable.

### Description

Adds single-instance forwarding, bootstrap states, OS lock awareness, close
review, graceful shutdown orchestration, childless recovery behavior, and
window geometry persistence using non-sensitive platform storage initially.

### Deliverables

- Authenticated user-only single-instance endpoint.
- Startup and readiness state machine.
- Window geometry and focus restoration.
- Close review and shutdown coordinator.
- OS session lock/unlock events and local recovery markers.

### Dependencies

- Milestones 04 and 06.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- A second launch can only send an allowed bounded launch intent.
- Startup and shutdown follow documented state order and deadlines.
- Window loss cannot authorize or silently continue an incomplete confirmation.
- Forced termination is detected on next startup without claiming clean exit.

### Security Notes

Authenticate the local endpoint, use user-only permissions, reject arbitrary
paths/commands, and clear confirmation/lease state on lock or webview loss.

### Testing Requirements

- Single-instance spoofing and malformed-intent tests.
- Startup failure, close cancellation, deadline escalation, forced-kill, and
  crash-recovery tests on Tier 1 platforms.

### Checklist

- [x] Implement single-instance ownership.
- [x] Implement bounded launch intents.
- [x] Implement startup state machine.
- [x] Persist safe window geometry.
- [x] Implement close review.
- [x] Implement structured shutdown.
- [x] Integrate OS lock awareness.
- [x] Add forced-termination recovery tests.

---

# Milestone 09 — Local PTY Runtime

### Status

- [ ] Not Started
- [ ] In Progress
- [ ] Blocked
- [x] Complete

### Goal

Start, supervise, resize, and stop one local shell safely from the Rust core.

### Description

Introduces platform PTY adapters, shell discovery, structured process
arguments, process-tree ownership, bounded input/output pumps, cancellation,
and a non-visual test session.

### Deliverables

- ConPTY and POSIX PTY adapters behind a common port.
- Shell profile discovery and explicit override.
- Session-owned child/process group supervision.
- Credit-based terminal byte stream and ordered input.
- Resize, exit, cancellation, deadline, and forced cleanup.

### Dependencies

- Milestones 02, 04, and 08.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- A local shell starts and exits cleanly on every Tier 1 platform.
- Input/output order is preserved under bounded memory.
- Resize works; child trees do not remain after normal or forced shutdown.
- Unsupported shell/platform states return actionable typed errors.

### Security Notes

Never concatenate shell command strings. Terminal output is untrusted bytes.
The app runs with user privileges and owns only children it created.

### Testing Requirements

- Fake PTY unit/integration tests.
- Platform process-tree, resize, cancellation, output-pressure, and orphan
  tests.
- Hostile terminal-byte fixtures without rendering.

### Checklist

- [x] Define PTY adapter contract.
- [x] Implement platform adapters.
- [x] Discover and validate shell profiles.
- [x] Supervise child/process tree.
- [x] Add bounded byte streams and input sequencing.
- [x] Add resize and exit handling.
- [x] Add graceful and forced cleanup.
- [x] Pass Tier 1 PTY conformance tests.

---

# Milestone 10 — Terminal Rendering

### Status

- [ ] Not Started
- [x] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Render and interact with the local PTY as a usable terminal.

### Description

Connects xterm.js to the renderer-neutral stream, preserving a terminal model
across DOM detachment and implementing safe input, selection, clipboard,
resize, Unicode, URI, and accessibility behavior.

### Deliverables

- xterm.js terminal model/view adapter.
- Stream credit, batching, replay, and explicit output-gap display.
- Input, resize, selection, and deliberate clipboard controls.
- Safe link/title/parser-hook policy.
- Font, IME, Unicode-width, and screen-reader baseline.

### Dependencies

- Milestones 05 and 09.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Users can complete a sustained local terminal session.
- Rendering never routes terminal bytes through general React state.
- Clipboard and URI actions require documented user intent.
- Detached/remounted rendering preserves bounded model state.
- Throughput, latency, and memory stay within initial budgets.

### Security Notes

Audit each xterm addon separately. Never use remote text as HTML, navigation,
trusted UI, or automatic clipboard/URI authority.

### Testing Requirements

- Terminal compatibility fixtures and common interactive-program smoke tests.
- Escape-sequence, title, link, clipboard, Unicode, IME, backpressure, and
  sustained-output tests.

### Checklist

- [ ] Add reviewed xterm.js dependency and required addons.
- [ ] Implement model/view separation.
- [ ] Connect bounded stream and credit flow.
- [ ] Implement input and resize.
- [ ] Implement safe selection and clipboard.
- [ ] Implement URI and title policy.
- [ ] Add Unicode, IME, and accessibility coverage.
- [ ] Pass latency and sustained-output budgets.

---

# Milestone 11 — Session Manager

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Manage multiple local session lifecycles without exposing runtime ownership to
the frontend.

### Description

Adds session metadata, create/start/detach/reattach/close states, session list,
active ownership, limits, typed failures, and explicit close policies.

### Deliverables

- Session domain state machine and supervisor registry.
- Session list and active-session status.
- Create, detach, reattach, close, and failure recovery actions.
- Global/profile limits and warning threshold.
- Per-session diagnostic summary.

### Dependencies

- Milestone 10.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- Multiple local sessions run independently within capacity limits.
- Each session reaches exactly one terminal state.
- Closing a view and closing a session are distinct operations.
- Webview loss cannot cause unbounded replay or orphaned processes.

### Security Notes

Session IDs convey no authority. Core policy validates every input, close, and
stream-open request against owner and state.

### Testing Requirements

- Complete state-table and invalid-transition tests.
- Concurrent create/close, detach, webview-loss, capacity, and shutdown tests.

### Checklist

- [ ] Define session states and transitions.
- [ ] Add session supervisor registry.
- [ ] Add session list and statuses.
- [ ] Add detach and reattach.
- [ ] Add close-policy review.
- [ ] Enforce warning and hard limits.
- [ ] Add state-table and concurrency tests.

---

# Milestone 12 — Terminal Tabs

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Organize multiple live sessions in persistent, keyboard-accessible tabs.

### Description

Adds tab creation, naming, switching, ordering, close behavior, status
indicators, and active-session focus without layout splitting.

### Deliverables

- Tab model referencing sessions.
- Tab strip, overflow behavior, status, rename, reorder, and close.
- Keyboard tab navigation and action-registry integration.
- Safe duplicate-name disambiguation.

### Dependencies

- Milestones 07 and 11.

### Estimated Complexity

**Medium**

### Acceptance Criteria

- Users can create, switch, rename, reorder, and close tabs entirely by
  keyboard.
- Tab close applies session ownership policy and names the affected process.
- Large tab sets remain searchable and do not break layout.

### Security Notes

Remote titles are untrusted suggestions and cannot silently rename trusted host
or environment context.

### Testing Requirements

- Tab model, ownership, keyboard, overflow, focus, and hostile-title tests.
- Visual regression for tab states and large tab sets.

### Checklist

- [ ] Define tab/session ownership model.
- [ ] Implement tab strip.
- [ ] Add create, switch, rename, and reorder.
- [ ] Add close review.
- [ ] Add keyboard shortcuts and palette actions.
- [ ] Add overflow and large-set behavior.
- [ ] Pass focus and hostile-title tests.

---

# Milestone 13 — Split Panes and Layout Model

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Support multi-pane terminal workflows with a bounded, valid layout tree.

### Description

Adds horizontal/vertical splits, focus movement, resize, move, close, zoom, and
an in-memory revisioned pane tree. Durable restoration follows after encrypted
persistence.

### Deliverables

- Bounded acyclic pane-tree model.
- Split, resize, focus, move, zoom, and close operations.
- Active-pane context and keyboard controls.
- Layout patch/revision contract.
- Responsive minimum-size behavior.

### Dependencies

- Milestone 12.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- A user can build and operate a ten-pane layout without losing target context.
- Invalid, stale, cyclic, or oversized layout patches are rejected.
- Focus and session ownership remain deterministic after every tree operation.
- Pane resizing does not flood PTY or persistence channels.

### Security Notes

Layout data cannot create commands, paths, remote connections, or trusted UI.
All trees and labels are bounded and validated.

### Testing Requirements

- Property tests for tree operations and invariants.
- Keyboard/focus, resize coalescing, capacity, and responsive E2E tests.

### Checklist

- [ ] Define pane-tree schema and limits.
- [ ] Implement split and close operations.
- [ ] Implement resize and move.
- [ ] Implement focus navigation and zoom.
- [ ] Add revisioned layout patches.
- [ ] Connect pane/session ownership.
- [ ] Pass property and ten-pane tests.

---

# Milestone 14 — Encrypted Profile and Persistence

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Persist Relio metadata securely through one transactional writer.

### Description

Selects the exact encrypted SQLite stack and implements profile keys, OS secret
store access, migrations, repositories, outbox, writer locking, integrity,
encrypted backups, recovery mode, and plaintext-canary verification.

### Deliverables

- ADR selecting provider, binding, crypto backend, build, and license path.
- SQLCipher-compatible database and one writer service.
- Profile root key in native OS secret stores.
- Migration, integrity, transactional outbox, backup, and recovery framework.
- Temporary non-persistent local-terminal mode when the profile cannot open.

### Dependencies

- Milestones 01–04 and 08.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Metadata persists across restart with no plaintext fallback.
- Database, WAL/journal, temporary, migration, and backup files contain no
  plaintext canaries.
- A second writer is excluded.
- Keychain denial, corruption, failed migration, interruption, and recovery
  remain safe and understandable.

### Security Notes

Secret bytes never cross frontend IPC, logs, command lines, environment
variables, or ordinary settings. Key loss is disclosed as permanent without a
usable backup.

### Testing Requirements

- Production encrypted-provider repository tests.
- Keychain conformance on Tier 1 platforms.
- Wrong-key, tamper, canary, migration, crash, backup, restore, and lock tests.

### Checklist

- [ ] Approve encrypted-database ADR and dependencies.
- [ ] Implement OS secret-store adapters.
- [ ] Implement profile key hierarchy.
- [ ] Implement single writer and repositories.
- [ ] Implement migrations and outbox.
- [ ] Implement encrypted backup and recovery mode.
- [ ] Implement writer/profile locking.
- [ ] Pass plaintext-canary and failure tests.

---

# Milestone 15 — Settings Engine and Settings UI

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Provide typed, scoped, explainable configuration without storing secrets.

### Description

Adds the settings schema, precedence, validation, revisioned transactions,
preview/revert, restart-required state, search, reset-at-scope, and a usable
settings interface.

### Deliverables

- Versioned settings schema and registry.
- Compiled/platform/profile/workspace/session precedence.
- Atomic multi-key set/reset and revision conflict handling.
- Safe preview overlay and restart-required reporting.
- Searchable settings UI showing effective value and source.
- Redacted versioned settings export.

### Dependencies

- Milestones 06, 07, and 14.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- Settings survive restart and resolve deterministically by scope.
- Invalid or unknown values never partially apply.
- Preview reverts on cancel, timeout, crash, or window loss.
- Secrets and immutable security policy cannot enter settings.

### Security Notes

Security, encryption, credential, and updater trust settings are not
previewable. Export excludes secret values and handles.

### Testing Requirements

- Schema, precedence, transaction, migration, revision, preview, export, search,
  keyboard, and accessibility tests.

### Checklist

- [ ] Define settings schema contract.
- [ ] Implement precedence and effective-value resolver.
- [ ] Implement transactional set/reset.
- [ ] Implement revision conflict handling.
- [ ] Implement preview and restart-required states.
- [ ] Build searchable settings UI.
- [ ] Add redacted export.
- [ ] Pass schema and lifecycle tests.

---

# Milestone 16 — Theme Engine and Theme Editor

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Allow safe local appearance customization without executable theme content.

### Description

Adds persisted bundled/user themes, complete token resolution, validation,
atomic preview/commit/fallback, workspace selection, and an accessible editor.

### Deliverables

- Versioned data-only theme schema and bundled presets.
- Validator for tokens, contrast, motion, fonts, and assets.
- Last-known-good and compiled fallback behavior.
- Window-scoped preview and transactional commit.
- Theme editor, reset, duplicate, delete, and workspace selection.

### Dependencies

- Milestones 05, 14, and 15.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- Valid themes apply atomically and persist; invalid themes cannot partially
  apply or block startup.
- Safety chrome and accessibility constraints remain invariant.
- Theme data cannot reference remote assets, scripts, CSS, HTML, or font files.
- Editor workflows are keyboard and screen-reader complete.

### Security Notes

Treat theme names and tokens as untrusted bounded data. V1 has no import,
marketplace, runtime code, or package format.

### Testing Requirements

- Schema, hostile-value, fallback, crash-preview, contrast, forced-color,
  reduced-motion, visual regression, and safety-chrome tests.

### Checklist

- [ ] Define versioned theme schema.
- [ ] Add bundled presets.
- [ ] Implement token resolution and validation.
- [ ] Implement preview, commit, and fallback.
- [ ] Persist global/workspace selection.
- [ ] Build accessible theme editor.
- [ ] Add reset, duplicate, and delete.
- [ ] Pass hostile-theme and visual tests.

---

# Milestone 17 — Workspace Manager and Durable Layout Restore

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Organize operational context into durable local workspaces.

### Description

Implements workspace CRUD, active/background/archive/delete states, references,
revisioned layout persistence, restore descriptors, impact review, search, and
redacted export.

### Deliverables

- Workspace aggregate and repository.
- Workspace switcher/manager with create, rename, archive, restore, and delete.
- Debounced transactional pane/tab layout persistence.
- Restorable local-session descriptors with honest reconnect/start language.
- Reference impact preview and redacted versioned export.

### Dependencies

- Milestones 13–15.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- Workspace metadata and valid layout survive restart.
- Restore creates new sessions only after user intent; process continuity is
  never claimed.
- Archive and delete resolve live owners and do not remove shared resources.
- Revision conflicts preserve authoritative structure.

### Security Notes

Workspace export contains no credential bytes/handles or active authority. V1
does not import workspace exports.

### Testing Requirements

- Aggregate, ownership, archive/delete, revision, crash persistence, malformed
  layout, large workspace, export, and restore-degradation tests.

### Checklist

- [ ] Implement workspace aggregate and repository.
- [ ] Build workspace manager and switcher.
- [ ] Persist revisioned layouts.
- [ ] Persist restorable session descriptors.
- [ ] Implement archive and restore.
- [ ] Implement impact-aware permanent delete.
- [ ] Implement redacted export.
- [ ] Pass restart and ownership tests.

---

# Milestone 18 — Host Manager

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Manage reusable remote host metadata independently from workspaces.

### Description

Adds global host profiles, workspace associations, aliases, groups, tags,
favorites, environment labels, safe validation, reference impact, and
capability placeholders without initiating SSH.

### Deliverables

- Host aggregate, repository, and revisioned commands.
- Host list/detail/create/edit/archive/delete UI.
- Groups, tags, favorites, environment, and workspace associations.
- Search/filter/virtualization for large inventories.
- Reference impact and unresolved-credential display.

### Dependencies

- Milestones 15 and 17.

### Estimated Complexity

**Medium**

### Acceptance Criteria

- Hosts can be reused across workspaces without duplicating credentials.
- Delete/archive clearly distinguishes Relio metadata from remote resources.
- Large reference datasets meet list/search budgets.
- Host fields remain structured data and never become shell fragments.

### Security Notes

Addresses and usernames are sensitive metadata. Exports and diagnostics redact
them according to policy.

### Testing Requirements

- Validation, revisions, associations, delete impact, large-list, hostile text,
  search, keyboard, and accessibility tests.

### Checklist

- [ ] Define host aggregate and repository.
- [ ] Build host list and details.
- [ ] Add create/edit/archive/delete.
- [ ] Add groups, tags, favorites, and environments.
- [ ] Add workspace associations and impact preview.
- [ ] Add scalable search/filter behavior.
- [ ] Pass hostile-field and reference tests.

---

# Milestone 19 — Credential Manager and Secret Service

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Register, use, rotate, revoke, and delete credential references without
exposing secret material to ordinary application state.

### Description

Implements opaque handles, purpose-bound leases, agent identities, external key
file registration, keychain-backed passwords/passphrases, OS reauthentication,
and credential metadata UI.

### Deliverables

- Rust secret service and native provider adapters.
- Opaque handles and non-serializable short-lived leases.
- Agent and external private-key reference registration.
- Credential manager with source, scope, status, references, rotation, revoke,
  and deletion.
- Lock/reauthentication and partial-failure behavior.

### Dependencies

- Milestones 14 and 18.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Secret bytes never enter frontend DTOs, database records, logs, exports,
  command arguments, or ordinary environment variables.
- File references are revalidated before use and never copied or deleted.
- Revocation removes future authority even if OS-store deletion fails.
- Provider lock/denial/unavailability has safe remediation.

### Security Notes

Review memory lifetime, clipboard behavior, secure input, same-user process
limits, and OS-specific access controls. No plaintext fallback.

### Testing Requirements

- Provider conformance; lease scope/expiry/replay; rotation rollback; lock,
  denial, deletion failure; path replacement; and secret-canary tests.

### Checklist

- [ ] Implement secret handles and lease policy.
- [ ] Implement native provider adapters.
- [ ] Implement agent identity registration.
- [ ] Implement external key-file registration and revalidation.
- [ ] Build credential manager.
- [ ] Add rotate, revoke, and delete workflows.
- [ ] Integrate reauthentication and OS lock.
- [ ] Pass secret absence and provider failure tests.

---

# Milestone 20 — OpenSSH Capability and Configuration Engine

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Diagnose and construct a safe, explainable OpenSSH connection plan.

### Description

Adds provider discovery/version support, safe-subset SSH configuration parsing,
bounded includes, generated protected configuration, ProxyJump planning,
algorithm policy, and effective-value diagnosis without authenticating.

### Deliverables

- Supported OpenSSH provider matrix and capability adapter.
- Safe SSH config parser with include limits and executable directives blocked.
- Effective connection-plan resolver and protected generated config.
- ProxyJump, agent, identity, algorithm, and unsupported-option diagnosis.
- Host detail capability and remediation UI.

### Dependencies

- Milestones 18–19.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Supported providers produce an immutable visible connection plan.
- Unknown/unavailable providers fail before connection with actionable detail.
- No config directive executes local commands or expands into shell text.
- Legacy exceptions are explicit, per-host, reviewable, and never global.

### Security Notes

SSH config is untrusted input. Disable `ProxyCommand`, `LocalCommand`,
`Match exec`, environment hooks, and unsupported executable behavior.

### Testing Requirements

- Tier 1 provider compatibility.
- Include cycle/depth/size, hostile alias/path/option, algorithm-policy,
  ProxyJump, generated-permission, and executable-directive denial tests.

### Checklist

- [ ] Define provider capability contract.
- [ ] Implement provider discovery and version diagnosis.
- [ ] Implement safe config parser.
- [ ] Implement bounded includes.
- [ ] Implement effective connection-plan resolver.
- [ ] Generate protected minimal config.
- [ ] Implement algorithm and legacy-exception policy.
- [ ] Build capability diagnostics UI.
- [ ] Pass hostile configuration tests.

---

# Milestone 21 — Host Identity and SSH Authentication

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Verify remote identity and authenticate through protected credential handoff.

### Description

Implements Relio-managed known hosts, controlled read-only sources, first-seen
review, changed/revoked blocking, fingerprint history, authenticated askpass,
agent selection, and explicit agent-forwarding consent.

### Deliverables

- Known-host repository and verification service.
- Trusted first-seen/changed/revoked review UI.
- Fingerprint evidence and verification history.
- One-time user-only askpass helper channel.
- Agent/key/password authentication and forwarding consent.
- Expiring host test-connection result.

### Dependencies

- Milestones 19–20.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Unknown keys require explicit trust; changed/revoked keys block by default.
- Authentication answers never appear in args, ordinary environment, IPC, or
  logs.
- Test connection authenticates and probes, then disconnects without creating a
  durable session or silently mutating trust.
- Agent forwarding is disabled by default and scoped to visible target/chain.

### Security Notes

The askpass path must resist spoofing, replay, oversized prompts, stale
channels, and helper crashes. `ssh-keyscan` alone is never identity proof.

### Testing Requirements

- Unknown/changed/revoked/hashed known-host cases.
- Askpass spoof/replay/timeout/cancel tests.
- Authentication failure classification and secret canaries.
- Agent-forwarding consent and reconnect behavior.

### Checklist

- [ ] Implement known-host repository.
- [ ] Implement verification policy and history.
- [ ] Build trusted fingerprint review.
- [ ] Implement authenticated askpass channel.
- [ ] Integrate credential leases and agents.
- [ ] Implement forwarding consent.
- [ ] Implement test connection.
- [ ] Pass host-identity and helper attack tests.

---

# Milestone 22 — SSH Connection and Remote Session Manager

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Open and supervise interactive SSH sessions with explicit lifecycle behavior.

### Description

Connects the OpenSSH plan and authentication service to terminal sessions,
including jump chains, connection progress, interruption, bounded retry,
disconnect, reconnect, and remote diagnostics.

### Deliverables

- SSH transport adapter integrated with session supervisor.
- Connection lifecycle and progress events.
- Interactive remote terminal sessions.
- Explicit disconnect/reconnect with bounded transient retry.
- Jump-chain and active identity/algorithm details.
- Typed remote connection diagnostics.

### Dependencies

- Milestones 11 and 20–21.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Supported hosts open usable interactive terminal sessions on Tier 1 targets.
- Retry never weakens route, identity, algorithm, or credential policy.
- Reconnect creates a new transport and never replays terminal input.
- Child processes/channels are cancelled, reaped, and reflected exactly once.

### Security Notes

Visible target, environment, identity, jump chain, host-key source, and
forwarding state remain available. Raw stderr is diagnostic input only.

### Testing Requirements

- Controlled SSH server integration tests.
- Jump failure, auth rejection, interruption, bounded retry, reconnect,
  cancellation, webview loss, and process cleanup tests.

### Checklist

- [ ] Implement SSH transport adapter.
- [ ] Connect connection/session state machines.
- [ ] Add interactive remote terminal.
- [ ] Add connection progress and details.
- [ ] Add bounded transient retry.
- [ ] Add explicit disconnect/reconnect.
- [ ] Normalize diagnostics.
- [ ] Pass controlled-server and cleanup tests.

---

# Milestone 23 — Durable Session Restore

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Restore workspace/session intent honestly after restart.

### Description

Persists local and SSH session descriptors, renders restorable placeholders,
revalidates capabilities and references, and lets users deliberately start or
reconnect selected sessions.

### Deliverables

- Versioned restorable descriptor records.
- Placeholder states for local and SSH sessions.
- Capability/reference revalidation and degraded remediation.
- Per-session and workspace reconnect/start actions.
- Crash/interrupted-session reconciliation.

### Dependencies

- Milestones 17 and 22.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- Restart restores layout, labels, target context, and descriptors—not PIDs,
  channels, terminal bytes, or implied process continuity.
- No remote connection begins during startup.
- Missing hosts, credentials, shells, or providers degrade visibly.
- User-selected restore respects capacity and current security policy.

### Security Notes

Descriptors contain references and metadata only. Restored intent never bypasses
new host-key, credential, forwarding, or algorithm decisions.

### Testing Requirements

- Clean/forced restart, missing reference, changed capability, capacity,
  selective reconnect, and no-startup-network tests.

### Checklist

- [ ] Define descriptor schema.
- [ ] Persist descriptors transactionally.
- [ ] Render restorable placeholders.
- [ ] Revalidate dependencies on restore.
- [ ] Add selected and workspace restore actions.
- [ ] Reconcile interrupted sessions.
- [ ] Pass no-automatic-reconnect tests.

---

# Milestone 24 — Bounded SFTP Transport

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Provide a safe structured remote filesystem protocol over authenticated SSH.

### Description

Implements a bounded SFTP client over a separately supervised OpenSSH subsystem
connection, with packet parsing, request correlation, paging, path-byte
preservation, capabilities, cancellation, and timeouts.

### Deliverables

- SFTP subsystem supervisor and binary protocol adapter.
- Bounded packet parser and request correlation.
- Remote path and metadata types.
- Directory paging and core file operations.
- Capability, timeout, cancellation, and typed failure model.
- Explicit legacy SCP refusal and diagnosed SFTP-semantic SCP capability.

### Dependencies

- Milestones 20–22.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Directory and file metadata operations work through structured requests.
- Packet, outstanding-request, path, page, buffer, and timeout limits hold.
- Non-text paths round-trip in the core and display loss is marked.
- Legacy SCP cannot be activated on any supported platform.

### Security Notes

Do not parse human-readable `sftp` output or use remote-shell paths. Treat the
server and every packet/name as hostile.

### Testing Requirements

- Controlled SFTP server and protocol conformance.
- Fuzz malformed/oversized/truncated packets and request IDs.
- Timeout, cancellation, hostile name, non-text path, symlink, and legacy-SCP
  refusal tests.

### Checklist

- [ ] Define SFTP port and capability model.
- [ ] Supervise separate subsystem process.
- [ ] Implement bounded packet framing.
- [ ] Implement request correlation and timeouts.
- [ ] Implement path-byte and metadata types.
- [ ] Implement paged directory operations.
- [ ] Diagnose SFTP-semantic SCP capability.
- [ ] Pass protocol fuzz and refusal tests.

---

# Milestone 25 — File Transfer Manager

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Upload and download files with explicit target, conflict, progress, verification,
and recovery behavior.

### Description

Adds transfer preflight, queueing, limits, temporary destinations, overwrite
decisions, progress, pause/resume when provable, verification, promotion,
cancellation, and interrupted-transfer recovery.

### Deliverables

- Transfer domain state machine and supervisor.
- Transfer queue/list/detail UI.
- SFTP upload/download and SFTP-semantic SCP workflow.
- Conflict review: skip, rename, or confirmed replace.
- Verification, atomic promotion disclosure, cleanup, and interrupted records.

### Dependencies

- Milestone 24.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Transfers never overwrite by default or accept a partial file as complete.
- Active/host/queue limits and progress coalescing are enforced.
- Pause/resume appears only when identities can be revalidated.
- Restart never resumes automatically and identifies owned temporary files.

### Security Notes

Bind user intent to exact host, source, destination, direction, symlink policy,
and expected identity. Never infer safety from a filename.

### Testing Requirements

- Upload/download, conflict, replace, rename, cancellation, interruption,
  disk-full, identity-change, symlink-race, atomic/non-atomic, cleanup, and
  concurrency tests.

### Checklist

- [ ] Define transfer states and preflight.
- [ ] Implement queue and supervisors.
- [ ] Implement upload and download.
- [ ] Implement conflict decisions.
- [ ] Implement temporary destination and promotion.
- [ ] Implement verification.
- [ ] Implement capability-gated pause/resume.
- [ ] Implement interruption recovery and cleanup.
- [ ] Pass destructive-path and concurrency tests.

---

# Milestone 26 — Remote File Browser

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Navigate remote files and launch safe file operations from the workbench.

### Description

Builds a paged, virtualized browser over SFTP metadata with breadcrumbs,
sorting, refresh, permissions, ownership, symlink visibility, download/upload,
rename, create directory, and confirmed deletion.

### Deliverables

- Remote browser pane and navigation model.
- Paged/virtualized directory listing and refresh.
- Metadata/permission/symlink inspector.
- File operation actions using structured SFTP contracts.
- Loading, empty, stale, permission, disconnect, and error states.

### Dependencies

- Milestones 06 and 24–25.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- Users can browse large directories without unbounded memory or UI blocking.
- Every operation shows exact host and path.
- Non-round-trippable names remain operable by stable opaque identity.
- Delete/rename/create actions handle stale metadata and conflicts explicitly.

### Security Notes

Remote names are untrusted display data and cannot become HTML, options, shell
fragments, routes, or trusted prompts.

### Testing Requirements

- Paging, virtualization, refresh, stale state, hostile/non-text names,
  permissions, symlinks, disconnect, keyboard, and accessibility tests.

### Checklist

- [ ] Implement browser navigation model.
- [ ] Build paged virtualized listing.
- [ ] Add breadcrumbs and refresh.
- [ ] Add metadata inspector.
- [ ] Connect upload/download actions.
- [ ] Add rename/create/delete flows.
- [ ] Handle stale/disconnected/error states.
- [ ] Pass hostile-name and large-directory tests.

---

# Milestone 27 — Remote Text Editor

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Safely edit small remote text files with conflict-aware saves.

### Description

Adds bounded UTF-8 plain-text opening, memory-only dirty buffers, line-ending
preservation, version tokens, revalidation, temporary upload, atomic promotion,
conflict resolution, save-as, reload, and close review.

### Deliverables

- Remote editor pane and memory-only buffer controller.
- File type/encoding/size checks with 10 MiB hard limit.
- Strongest-available remote version token.
- Save, save-as, reload, overwrite, discard, and conflict flows.
- Atomicity/permission preservation reporting.

### Dependencies

- Milestones 25–26.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Binary, NUL-containing, invalid UTF-8, oversized, and unsupported targets are
  refused for editing.
- A changed remote identity never receives a silent last-write-wins save.
- Unsaved content is absent from database, logs, crash reports, and recordings.
- Dirty close and app shutdown require save, discard, or cancel.

### Security Notes

Render content as plain text only. No external editor handoff, plaintext draft,
HTML preview, or automatic crash recovery in v1.

### Testing Requirements

- Encoding/size/type refusal, version conflict, symlink race, permissions,
  atomic/non-atomic save, crash absence, dirty shutdown, and memory-limit tests.

### Checklist

- [ ] Implement bounded text open.
- [ ] Implement memory-only buffer lifecycle.
- [ ] Capture and revalidate version tokens.
- [ ] Implement temporary upload and promotion.
- [ ] Implement conflict, reload, overwrite, and save-as.
- [ ] Implement dirty close/shutdown review.
- [ ] Add permission and atomicity disclosure.
- [ ] Pass plaintext-absence and conflict tests.

---

# Milestone 28 — Port Forwarding Manager

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Create and supervise clear, auditable SSH tunnels.

### Description

Adds local, remote, and dynamic forwarding models, preflight, loopback defaults,
broad-bind confirmation, owned listener/process lifecycle, workspace
association, status, stop/restart, and conflict diagnosis.

### Deliverables

- Tunnel aggregate and lifecycle supervisor.
- Create/edit/start/stop/restart/list UI.
- Local, remote, and dynamic forwarding.
- Endpoint, jump chain, owner, bind scope, and state display.
- Owned control-socket/listener cleanup and reconnect reconciliation.

### Dependencies

- Milestones 17 and 22.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Loopback is the default; broad binds require exact trusted confirmation.
- One desired tunnel owns at most one active listener.
- Stop verifies the owned resource ended and never kills by port number alone.
- Crash/restart does not silently recreate or duplicate a listener.

### Security Notes

Broad binds may require OS reauthentication. Display both endpoints, transport
host, direction, jump chain, and exposure before start.

### Testing Requirements

- Local/remote/dynamic behavior, bind conflict, privileged port, broad consent,
  reconnect duplication, cancellation, crash, and orphan cleanup tests.

### Checklist

- [ ] Define tunnel states and endpoints.
- [ ] Implement preflight and structured plans.
- [ ] Implement local forwarding.
- [ ] Implement remote forwarding.
- [ ] Implement dynamic forwarding.
- [ ] Build manager UI.
- [ ] Add broad-bind confirmation.
- [ ] Add owned cleanup and reconciliation.
- [ ] Pass exposure and orphan tests.

---

# Milestone 29 — Command Snippets

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Reuse reviewed commands without hidden execution.

### Description

Adds scoped one-line snippets, tags, parameters, validation, preview, target
display, search, insertion into the active terminal, and explicit proof that no
synthetic submission occurs.

### Deliverables

- Snippet aggregate, repository, scopes, tags, and revisions.
- Snippet manager and parameter editor.
- Bounded parameter prompts and complete command preview.
- Active-session target/identity display.
- Insert-only terminal action and palette integration.

### Dependencies

- Milestones 07, 15, 17, and 22.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- Snippets cannot contain hidden newlines or disallowed control characters.
- Parameter expansion is bounded and fully previewed.
- Insertion targets the visibly active session and never generates Enter.
- Snippets contain no secret values or automatic credential expansion.

### Security Notes

Treat snippet text as untrusted terminal input requiring user review. Never
interpret it as a shell command inside the core.

### Testing Requirements

- Validation, parameter bounds, revisions, scope, target focus, hostile Unicode,
  control-character, and no-synthetic-Enter tests.

### Checklist

- [ ] Define snippet schema and scopes.
- [ ] Implement repository and revisions.
- [ ] Build snippet manager.
- [ ] Implement bounded parameters.
- [ ] Implement complete preview.
- [ ] Display active target and identity.
- [ ] Implement insert-only action.
- [ ] Pass hidden-input and submission tests.

---

# Milestone 30 — Opt-In Command History

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Provide useful derived command recall without silently retaining terminal
content.

### Description

Adds explicit retention enablement, bounded shell-integration/derived command
records, uncertainty disclosure, encrypted storage, search, deletion, expiry,
and reviewed insertion.

### Deliverables

- History policy, schema, retention, and encrypted repository.
- Explicit per-profile/workspace/session enablement.
- Derived command-boundary integration with confidence/source.
- History list/search/delete UI.
- Insert-only reuse through the snippet safety path.

### Dependencies

- Milestones 14, 15, 22, and 29.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- History is off until explicitly enabled.
- Derived records never alter terminal truth or claim completeness.
- Retention and deletion are visible and testable.
- Reuse never submits automatically or hides control characters.

### Security Notes

Commands may contain secrets. Encryption and best-effort redaction do not make
retention harmless; warn before enablement and support short retention.

### Testing Requirements

- Opt-in/out, parsing uncertainty, encryption canary, retention, deletion,
  redaction, bounded indexing, and insert-only tests.

### Checklist

- [ ] Define history schema and policy.
- [ ] Add explicit enablement controls.
- [ ] Integrate bounded command derivation.
- [ ] Persist encrypted metadata.
- [ ] Build history list and search.
- [ ] Add retention and deletion.
- [ ] Connect safe reviewed insertion.
- [ ] Pass privacy and no-auto-submit tests.

---

# Milestone 31 — Search Everywhere

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Find local Relio metadata quickly from one bounded, cancellable interface.

### Description

Adds indexed and federated search across workspaces, hosts, sessions, snippets,
history, recordings metadata, settings, themes, and actions, respecting
retention and lock state.

### Deliverables

- Search service, query contract, ranking, stable cursors, and cancellation.
- Bounded indexes for opted-in data only.
- Search Everywhere UI with filters, categories, keyboard navigation, and
  result actions.
- Index migration/rebuild and stale-result handling.

### Dependencies

- Milestones 07, 15–18, and 29–30.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- First-page search meets the reference budget over capacity datasets.
- Locked, deleted, archived, expired, or non-retained content is not leaked.
- Search cancellation and index rebuild remain bounded and recoverable.
- Result actions revalidate current entity revision and authority.

### Security Notes

Index only data the user elected to retain. Search results are display data, not
authorization, and cannot expose secret handles.

### Testing Requirements

- Ranking, paging, cancellation, migration, rebuild, deletion propagation,
  lock-state, retention, large-dataset, keyboard, and performance tests.

### Checklist

- [ ] Define search contract and result types.
- [ ] Implement bounded metadata indexes.
- [ ] Implement ranking and stable cursors.
- [ ] Build Search Everywhere UI.
- [ ] Add filters and keyboard actions.
- [ ] Add cancellation and stale-result handling.
- [ ] Add rebuild/migration behavior.
- [ ] Pass privacy and performance tests.

---

# Milestone 32 — Structured Logging, Diagnostics, and Log Viewer

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Make Relio failures diagnosable without turning logs into another secret store.

### Description

Implements encrypted structured operational logs, security audit facts,
redaction, rotation, retention, health diagnostics, connection/operation
correlation, log viewer, and previewed local support-bundle export.

### Deliverables

- Structured log/audit schemas and redaction service.
- Encrypted rotating sinks with separate retention.
- Health and diagnostic status service.
- Searchable paged log viewer.
- Allowlisted support-bundle preview and local export.

### Dependencies

- Milestones 04, 14–15, and 22.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- Secret canaries, raw terminal/file content, credential handles, and private
  path/host values do not appear in normal logs.
- Log failure does not crash sessions; required audit failure blocks only
  policy-designated operations.
- Support bundles list content before export and have no built-in upload.
- Retention, rotation, deletion, and disk reserve are enforced.

### Security Notes

Redaction is best effort and never a reason to log unnecessary sensitive data.
Stable debug mode cannot disable redaction.

### Testing Requirements

- Canary, redaction, pseudonymization, rotation, retention, disk-full,
  corruption, bundle allowlist/preview, paging, and failure-isolation tests.

### Checklist

- [ ] Define structured log and audit schemas.
- [ ] Implement redaction and pseudonymous correlation.
- [ ] Implement encrypted rotating sinks.
- [ ] Add retention and disk reserve.
- [ ] Implement health diagnostics.
- [ ] Build log viewer.
- [ ] Build support-bundle preview/export.
- [ ] Pass secret-canary and failure tests.

---

# Milestone 33 — Session Recording

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Record selected terminal sessions locally with explicit consent and bounded
encrypted storage.

### Description

Adds opt-in per-session recording, immutable authenticated segments, indexes,
status, quota, retention, playback/search metadata, deletion, export, and crash
finalization.

### Deliverables

- Versioned encrypted recording format and ADR/library decision.
- Recording sink independent from live rendering.
- Per-session controls and persistent visible indicator.
- Segment index, quota, retention, playback, deletion, and explicit export.
- Interrupted-segment recovery and tamper handling.

### Dependencies

- Milestones 10, 14–15, 31–32.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Recording is off by default and never required for live terminal correctness.
- Ciphertext, metadata, WAL/temp, and exports satisfy canary/integrity policy.
- Quota/disk reserve stops safely without breaking the live session.
- Tampered or truncated segments never return partial plaintext as valid.

### Security Notes

Recordings can contain secrets redaction misses. Consent, persistent status,
retention, deletion limitations, export preview, and local-only behavior must
remain clear.

### Testing Requirements

- Format vectors, wrong key, tamper, truncation, interrupted write, quota,
  retention, deletion, playback, export, recording/render independence, and
  canary tests.

### Checklist

- [ ] Approve recording format/library decision.
- [ ] Implement encrypted immutable segments.
- [ ] Add independent recording sink.
- [ ] Add session controls and indicator.
- [ ] Implement index and playback.
- [ ] Add quota, retention, and disk reserve.
- [ ] Add deletion and explicit export.
- [ ] Implement interrupted-segment recovery.
- [ ] Pass crypto and privacy tests.

---

# Milestone 34 — Cross-Platform Compatibility Hardening

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Bring all implemented v1 capabilities to consistent, explicit Tier 1 behavior.

### Description

Closes Windows, macOS, and Linux adapter gaps and publishes supported,
experimental, unavailable, and Tier 2 behavior without adding features.

### Deliverables

- Completed Tier 1 capability and exception matrix.
- Platform adapter conformance for PTY, keychain, paths, webviews, OpenSSH,
  files, processes, windows, and packaging prerequisites.
- Platform-specific diagnostics and degraded behavior.
- International input, filesystem, display-scale, and window-system evidence.

### Dependencies

- Milestones 01–33.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Every v1 capability has an evidence-backed Tier 1 state.
- Platform differences are visible and never silently weaken security.
- Tier 1 conformance suites pass; Tier 2 results and limitations are published.
- No platform-specific orphan, path corruption, or profile compatibility blocker remains.

### Security Notes

Keychain, ACL, process, path, signing, webview, and OpenSSH differences must fail
closed when their security guarantees are unavailable.

### Testing Requirements

- Full Tier 1 adapter and compatibility matrix.
- Filesystem/path, process-tree, keychain, webview, input, display, window, and
  capability-degradation tests.

### Checklist

- [ ] Complete Windows adapter matrix.
- [ ] Complete macOS adapter matrix.
- [ ] Complete Linux adapter matrix.
- [ ] Verify OpenSSH and keychain provider ranges.
- [ ] Verify filesystem and process semantics.
- [ ] Verify webview, input, scaling, and window behavior.
- [ ] Publish Tier 2 results and exceptions.
- [ ] Resolve Tier 1 blockers.

---

# Milestone 35 — Accessibility Completion

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Make the complete v1 experience operable with keyboard and assistive technology.

### Description

Audits and repairs semantics, focus, announcements, contrast, text scaling,
forced colors, reduced motion, input methods, and complete non-pointer workflows.

### Deliverables

- WCAG 2.2 AA audit and resolved findings.
- Complete keyboard workflow map.
- Screen-reader labels, state announcements, and focus restoration.
- High-contrast, forced-color, reduced-motion, and text-scale verification.
- Accessibility statement and known-limitations record.

### Dependencies

- Milestones 01–34.

### Estimated Complexity

**Hard**

### Acceptance Criteria

- Every core v1 workflow is completable without a pointer.
- Required content and trusted safety UI meet contrast and non-color rules.
- Dynamic session, transfer, tunnel, error, and recording states are announced.
- Supported text scaling and assistive technologies do not hide required context.

### Security Notes

Accessibility alternatives must preserve target, identity, risk, confirmation,
and status information rather than bypassing safety interactions.

### Testing Requirements

- Automated accessibility checks plus manual keyboard and Tier 1 screen-reader
  audits.
- Focus, announcement, forced-color, reduced-motion, zoom, and text-scale tests.

### Checklist

- [ ] Audit semantic structure and names.
- [ ] Verify complete keyboard workflows.
- [ ] Verify focus order and restoration.
- [ ] Add dynamic state announcements.
- [ ] Verify contrast and non-color cues.
- [ ] Verify forced colors and reduced motion.
- [ ] Verify text scaling and zoom.
- [ ] Resolve audit blockers and publish statement.

---

# Milestone 36 — Performance and Resource Hardening

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Meet the documented v1 performance and capacity budgets under realistic load.

### Description

Profiles startup, terminal latency/throughput, idle cost, memory, search,
database, files, recording, and long-running operation behavior on reference
systems, then removes leaks and unbounded work.

### Deliverables

- Reproducible performance datasets and reference-machine reports.
- Startup, terminal, workbench, search, database, transfer, and recording results.
- Long-session soak and resource-ownership report.
- Regression thresholds and CI trend capture.
- Documented disposition for every budget regression.

### Dependencies

- Milestones 01–35.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- All release budgets pass on published baseline systems.
- Queues, caches, streams, tasks, children, temporary files, and indexes remain bounded.
- Idle CPU and memory stabilize after workload completion.
- There is no startup network work or unexpected remote operation.

### Security Notes

Performance changes may not disable validation, encryption, redaction,
backpressure, confirmation, or cleanup. Output gaps must remain explicit.

### Testing Requirements

- Release-build benchmarks, capacity datasets, sustained-output and idle tests,
  long-session soak, fault injection, leak detection, and regression comparison.

### Checklist

- [ ] Record reference environments and datasets.
- [ ] Measure startup and first prompt.
- [ ] Measure terminal input, output, idle CPU, and memory.
- [ ] Measure workbench, search, database, and remote files.
- [ ] Measure transfer and recording behavior.
- [ ] Run long-session soak and leak detection.
- [ ] Add CI trend thresholds.
- [ ] Resolve or formally disposition every regression.

---

# Milestone 37 — Native Packaging

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Produce installable, removable, verifiable native packages for Tier 1 targets.

### Description

Creates target-specific package configuration and validates clean install,
launch, upgrade preparation, data-directory retention, and uninstall behavior
without implementing network updates.

### Deliverables

- Native Windows, macOS, and supported Linux packages.
- Development/nightly/stable identifier and data-path separation.
- Clean protected build inputs and artifact identity metadata.
- Package smoke, install, repair, and uninstall procedures.
- Checksums, license notices, and package-content inventory.

### Dependencies

- Milestones 01–36.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Clean Tier 1 machines can verify, install, launch, and uninstall packages.
- Uninstall behavior clearly preserves or removes profile data according to user choice.
- Packages contain only expected reviewed runtime files and capabilities.
- Build artifacts are target-specific and reproducibly attributable to source.

### Security Notes

Packaging jobs have no publishing/signing authority. Native dependencies,
install scripts, permissions, ACLs, and data paths require review.

### Testing Requirements

- Clean-machine package-content, install, launch, repair, uninstall, data
  retention, permission, and identifier-isolation tests.

### Checklist

- [ ] Configure Windows package.
- [ ] Configure macOS package.
- [ ] Configure supported Linux packages.
- [ ] Separate channel identifiers and data paths.
- [ ] Add package-content inventory.
- [ ] Add clean install and launch tests.
- [ ] Add uninstall and data-retention tests.
- [ ] Publish checksums and notices for test artifacts.

---

# Milestone 38 — Secure Update and Artifact Promotion

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Update direct-download installations through a protected, rollback-aware path.

### Description

Implements signed update metadata, check/download/staging, target and freshness
validation, immutable artifact promotion, OS signature verification, migration
preflight, health check, rollback, and Linux distribution-manager boundaries.

### Deliverables

- Protected build/sign/publish separation and immutable promotion pipeline.
- Platform signatures, Relio update signatures, SBOM, and provenance.
- Rust-owned update check, download, verification, staging, and install flow.
- Migration-aware health check and data-aware rollback.
- Signing/update key rotation, loss, compromise, and recovery runbooks.

### Dependencies

- Milestones 01–37.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Tested artifacts are the exact immutable artifacts signed and promoted.
- The updater rejects wrong key, target, channel, version, length, digest,
  expiry, replay, downgrade, redirect, OS signature, and staged tampering.
- Failed install/migration preserves a known-good application and recovery path.
- Distribution-managed Linux packages never self-modify outside their manager.

### Security Notes

Signing keys never enter source, ordinary CI, or developer workstations. Rehearse
rotation, compromise, maintainer departure, freeze, and out-of-band recovery.

### Testing Requirements

- Signature/metadata negative matrix; interrupted/disk-full download; staging
  tamper; migration failure; health-check; rollback; rotation; and package-manager tests.

### Checklist

- [ ] Separate protected build, signing, and publishing roles.
- [ ] Generate platform and update signatures.
- [ ] Generate SBOM and provenance.
- [ ] Implement update check and download.
- [ ] Implement full metadata/artifact verification.
- [ ] Implement protected staging and install.
- [ ] Implement health check and data-aware rollback.
- [ ] Rehearse key rotation, loss, and compromise.
- [ ] Pass updater negative matrix.

---

# Milestone 39 — Stable v1 Release Readiness

### Status

- [x] Not Started
- [ ] In Progress
- [ ] Blocked
- [ ] Complete

### Goal

Prove Relio v1 is safe, supportable, documented, and ready for stable promotion.

### Description

Closes release blockers, independent reviews, migration paths, operations
runbooks, support commitments, user/contributor documentation, and final
artifact verification without adding product features.

### Deliverables

- Independent security and accessibility review dispositions.
- Final privacy, threat-model, dependency, license, and support reviews.
- Release notes, support matrix, migration/rollback guidance, and known issues.
- Incident, vulnerability, key recovery, rollback, and maintainer-departure runbooks.
- Verified stable artifacts and signed metadata promoted from tested candidates.

### Dependencies

- Milestones 01–38.

### Estimated Complexity

**Very Hard**

### Acceptance Criteria

- Every prior milestone is complete with linked evidence.
- No unresolved critical/high security, data-loss, migration, Tier 1,
  accessibility, performance, licensing, signing, or support blocker remains.
- A clean Tier 1 machine can verify, install, launch, upgrade, recover, roll
  back, and uninstall using published documentation.
- Stable support ownership and private reporting paths are live and rehearsed.

### Security Notes

A validly signed malicious release remains catastrophic; require protected tags,
independent promotion approval, provenance verification, and recovery rehearsal.

### Testing Requirements

- Full release-candidate matrix across security, privacy, compatibility,
  accessibility, performance, migration, packaging, update, rollback, and documentation.

### Checklist

- [ ] Resolve independent security review findings.
- [ ] Resolve accessibility review findings.
- [ ] Complete privacy and threat-model review.
- [ ] Complete dependency, license, SBOM, and provenance review.
- [ ] Verify migration, recovery, and rollback paths.
- [ ] Rehearse incident, key recovery, and maintainer departure.
- [ ] Publish support matrix, release notes, and known issues.
- [ ] Verify all user, contributor, and operations documentation.
- [ ] Verify final artifacts and signed metadata.
- [ ] Promote tested artifacts to stable.

---

## Development Rules

1. Read this document before every implementation task.
2. Never implement more than one milestone in a single prompt, branch, or pull
   request.
3. Never skip milestones. If a later milestone appears necessary, document the
   dependency and finish the current milestone or mark it blocked.
4. Never introduce features, schemas, permissions, abstractions, dependencies,
   or compatibility promises reserved for future milestones.
5. Start a milestone only when all listed dependencies are complete.
6. Use one tracking issue and one focused pull request per milestone.
7. Select exactly one status checkbox per milestone.
8. Mark checklist items complete only after implementation and relevant tests
   succeed.
9. A milestone is complete only when its acceptance criteria, security notes,
   testing requirements, documentation, formatting, linting, and all required
   CI checks pass.
10. Link test evidence, measurements, screenshots where relevant, dependency
    reviews, and security decisions from the milestone issue or pull request.
11. If implementation reveals missing, conflicting, or incorrect
    documentation, stop implementation and update the authoritative
    documentation before continuing.
12. If the implementation must change a trust boundary, dependency direction,
    data owner, public contract, persisted format, platform promise, or release
    policy, create or supersede an ADR.
13. Keep commits internally coherent and scoped to the active milestone.
14. Preserve existing user changes and do not refactor unrelated modules.
15. Treat security, accessibility, cancellation, errors, lifecycle cleanup,
    platform behavior, performance limits, and documentation as feature work.
16. Do not weaken a test or security control merely to make a milestone pass.
17. Do not mark a milestone complete with known critical/high security,
    data-loss, migration, process-leak, or Tier 1 release-blocking defects.
18. Update the milestone status, checklist, overall completed count, and
    percentage in the same pull request that completes or reopens a milestone.
19. Compute progress as `complete milestones / 39 × 100`, rounded to the nearest
    whole percent.
20. Every milestone must remain independently buildable, testable,
    reviewable, reversible where documented, and understandable to the next
    human or AI agent without relying on chat history.
