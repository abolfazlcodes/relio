# Development Roadmap

The roadmap is organized as independently usable milestones. Complexity is relative engineering effort for a small team that is still learning the desktop and systems portions of the stack; it is not a calendar promise.

## Phase 0 — Product and architecture foundation

**Goal:** Make the product understandable and safe to implement.

**Deliverables:** vision, competitor analysis, architecture decisions, module boundaries, security model, contribution workflow, roadmap, feature map, and initial issue templates.

**Dependencies:** none.

**Risks:** documenting assumptions as facts; choosing too many future abstractions.

**Learning objectives:** translate product goals into boundaries, evaluate desktop runtime tradeoffs, and write decision records.

**Estimated complexity:** Small, but high leverage.

**Definition of done:** a contributor can explain what the first usable application is, what is deferred, where code belongs, and what must not be trusted.

## Phase 1 — Application shell

**Goal:** Produce a launchable cross-platform desktop shell with a minimal workbench.

**Deliverables:** Tauri window, React/TypeScript frontend, Rust command bridge, navigation shell, error boundary, settings placeholder, basic diagnostics, packaging smoke test.

**Dependencies:** Phase 0.

**Risks:** platform webview differences, IPC shape churn, build tool friction.

**Learning objectives:** Tauri lifecycle, Rust/TypeScript IPC, desktop packaging, platform debugging.

**Estimated complexity:** Medium.

**Definition of done:** the app launches on Windows, Linux, and macOS development targets and shows a usable empty workspace without network access.

## Phase 2 — Local terminal and layout

**Goal:** Make the product useful as a local terminal.

**Deliverables:** PTY-backed local shell, xterm.js view, input/output streaming, resize, copy/paste, tabs, split panes, pane focus, close behavior, session metadata, graceful process cleanup, basic restore of layout.

**Dependencies:** Phase 1.

**Risks:** terminal compatibility, output backpressure, process leaks, Windows shell behavior.

**Learning objectives:** PTYs, streams, terminal protocols, process supervision, rendering performance.

**Estimated complexity:** Large.

**Definition of done:** users can work in multiple local panes for a full session, run interactive programs, and reopen the app with the previous layout restored.

## Phase 3 — Local data and settings

**Goal:** Make work persistent and inspectable.

**Deliverables:** SQLite migrations, repositories, workspace CRUD, settings scopes, schema validation, export/import with redaction, OS credential-store integration for a test secret.

**Dependencies:** Phase 1; session identifiers from Phase 2.

**Risks:** migration mistakes, data corruption, secret leakage, premature schema coupling.

**Learning objectives:** desktop data directories, migrations, keychain APIs, transactional persistence.

**Estimated complexity:** Medium.

**Definition of done:** settings and workspaces survive restart, migrations are tested, and exported data contains no plaintext credentials.

## Phase 4 — SSH and host manager

**Goal:** Connect reliably to real hosts.

**Deliverables:** host profiles, groups/tags/favorites, known-host review flow, SSH config import/read-through, agent/key selection, jump-host support, interactive SSH sessions, connection diagnostics.

**Dependencies:** Phases 2–3.

**Risks:** platform differences, authentication edge cases, unsafe host-key behavior, subprocess lifecycle.

**Learning objectives:** SSH configuration, authentication, host-key verification, remote PTYs, network failure handling.

**Estimated complexity:** Large.

**Definition of done:** a user can create or import a host, connect, see identity and verification state, and receive actionable errors without exposing secrets.

## Phase 5 — SFTP, file browser, and remote editing

**Goal:** Operate on remote files without leaving the workspace.

**Deliverables:** remote directory listing, upload/download, progress and cancellation, permissions display, local/remote file browser, explicit remote-edit save flow, conflict detection.

**Dependencies:** Phase 4.

**Risks:** partial transfers, file encoding, permission errors, accidental overwrites, symlink behavior.

**Learning objectives:** SFTP semantics, file identity, temporary files, conflict-safe writes.

**Estimated complexity:** Large.

**Definition of done:** users can browse and transfer files with visible target context and edit a remote file with an explicit, conflict-aware save.

## Phase 6 — Visual port forwarding

**Goal:** Replace error-prone tunnel command composition with an understandable manager.

**Deliverables:** local/remote/dynamic forwarding models, create/edit/stop/restart, bind-address warnings, state indicators, conflict diagnostics, workspace association.

**Dependencies:** Phase 4.

**Risks:** accidental exposure, orphaned listeners, reconnect behavior, privilege-required ports.

**Learning objectives:** socket lifecycle, forwarding semantics, safe defaults, cancellation.

**Estimated complexity:** Medium to large.

**Definition of done:** a user can create a tunnel, see exactly what it connects, verify state, and stop it without returning to a terminal command.

## Phase 7 — Infrastructure workspaces and detection

**Goal:** Organize a project’s hosts, services, and operating context.

**Deliverables:** workspace overview, host/service relationships, read-only detectors for Docker/Kubernetes/systemd/common runtimes, capability cards, safe next actions.

**Dependencies:** Phases 3–5.

**Risks:** false positives, remote command cost, permissions, vendor-specific assumptions.

**Learning objectives:** capability detection, remote inventory, caching, plugin/provider boundaries.

**Estimated complexity:** Large.

**Definition of done:** users can see a useful, clearly labeled overview of a host or project and understand which actions are available and why.

## Phase 8 — Workflows, history, recording, and search

**Goal:** Reduce repeated operational work and improve recall.

**Deliverables:** snippets, parameterized workflows, command history, session recording, log viewer, local search everywhere, retention controls, sensitive-output warnings.

**Dependencies:** Phases 2–7.

**Risks:** storing secrets in output, index size, command parsing ambiguity, search performance.

**Learning objectives:** event-derived data, indexing, privacy-aware UX, command lifecycle detection.

**Estimated complexity:** Large.

**Definition of done:** users can find a previous command or log event, reuse a reviewed snippet, and understand what data is stored locally.

## Phase 9 — Theme engine and plugin SDK

**Goal:** Let contributors extend and customize the product safely.

**Deliverables:** semantic theme schema, theme validation, plugin manifest, process host, capability grants, command/view/detector contributions, SDK examples, compatibility test harness.

**Dependencies:** Phases 1–8; stable IPC and operation contracts.

**Risks:** API churn, plugin crashes, permission confusion, arbitrary UI escape hatches.

**Learning objectives:** public API design, process isolation, protocol versioning, compatibility testing.

**Estimated complexity:** Very large.

**Definition of done:** an external contributor can build a documented plugin that adds a command or read-only detector without modifying the core repository.

## Phase 10 — Optional AI assistant

**Goal:** Add assistance without weakening local-first or execution safety.

**Deliverables:** provider interface, explicit context selection, explain-command, explain-error, summarize-approved-log, draft-command, redaction controls, no-implicit-execution policy.

**Dependencies:** Phases 8–9.

**Risks:** secret leakage, hallucinated commands, provider outages, cost, user over-trust.

**Learning objectives:** capability boundaries, privacy-preserving context, human-in-the-loop execution.

**Estimated complexity:** Large.

**Definition of done:** every AI response identifies its context and is clearly separated from executable terminal input.

## Phase 11 — Optional sync and marketplace

**Goal:** Add distribution and synchronization without making the desktop app cloud-dependent.

**Deliverables:** signed plugin/theme packages, compatibility metadata, optional catalog, allowlisted settings sync, conflict UI, offline fallback.

**Dependencies:** Phases 3 and 9; security review.

**Risks:** supply-chain attacks, account pressure, sync conflicts, secret handling.

**Learning objectives:** package signing, update channels, sync conflict models, service boundaries.

**Estimated complexity:** Very large.

**Definition of done:** disabling all network providers leaves the local product fully usable.

## Phase 12 — Hardening and release

**Goal:** Make the product trustworthy to install and maintain.

**Deliverables:** crash diagnostics with opt-in telemetry policy, performance budgets, accessibility audit, threat-model review, signed builds, platform installers, upgrade/migration tests, release notes, support matrix.

**Dependencies:** all previous phases selected for release.

**Risks:** packaging credentials, platform signing, upgrade regressions, support burden.

**Learning objectives:** release engineering, reproducible builds, distribution, incident response.

**Estimated complexity:** Large.

**Definition of done:** a clean machine can install, launch, upgrade, and uninstall a signed build on every supported platform with documented rollback guidance.

## Scope control

Each phase must deliver a narrow, usable slice. A new feature that does not fit the current phase should become a roadmap change or a follow-up issue, not an invisible expansion of the milestone.
