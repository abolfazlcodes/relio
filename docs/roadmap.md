# Development Roadmap

The roadmap is a sequence of independently usable milestones for a focused,
local-first v1. Complexity is relative engineering effort, not a calendar
promise. Every phase uses the same small trusted architecture; later phases do
not introduce a second application runtime or hosted control plane.

## Phase 0 — Product and architecture foundation

**Goal:** Make the focused product safe to implement.

**Deliverables:** reviewed product scope, technical blueprint, architecture
decisions, process and IPC model, workspace and persistence models, platform
policy, performance budgets, security documentation, contribution workflow,
roadmap, feature map, and issue templates.

**Dependencies:** none.

**Primary risks:** documenting assumptions as facts, speculative abstractions,
unclear ownership, or a security boundary the target operating systems cannot
enforce.

**Definition of done:** a contributor can explain the v1 feature boundary, data
owners, trust boundaries, release gates, and deferred concepts. Licensing,
provisional Tier 1 platforms, private security reporting, release-signing
ownership, performance reference systems, and security-critical dependency
licenses are resolved before implementation dependencies are accepted.

## Phase 1 — Application shell and command framework

**Goal:** Produce a launchable cross-platform desktop shell.

**Deliverables:** Tauri window, React/TypeScript workbench, Rust command bridge
with generated types, explicit per-window capabilities, restrictive content
security policy, navigation shell, searchable command registry, semantic design
tokens, error boundary, local diagnostics, and packaging smoke tests.

**Dependencies:** Phase 0.

**Primary risks:** platform webview differences, over-broad IPC, build friction,
and premature workbench complexity.

**Definition of done:** release builds launch on every provisional Tier 1
target, show a usable empty workspace without a startup network request, expose
no generic filesystem/shell/network command, pass content-policy and capability
tests, and record startup performance on reference systems.

## Phase 2 — Local terminal, tabs, and split panes

**Goal:** Make Relio useful as a local terminal.

**Deliverables:** PTY-backed shell, xterm.js model/view separation, bounded
input/output streams, resize, deliberate clipboard policy, tabs, split panes,
focus and close behavior, session metadata, process-tree cleanup, bounded
frontend-disconnect replay, and layout restore.

**Dependencies:** Phase 1.

**Primary risks:** terminal compatibility, hostile escape sequences,
backpressure, process leaks, and Windows process-tree behavior.

**Definition of done:** users can work in multiple local panes for a full
session and reopen the previous layout. Compatibility, output-gap, orphan
cleanup, memory, input-latency, and terminal-protocol abuse tests pass.

## Phase 3 — Encrypted local data, workspaces, and credentials

**Goal:** Make local work durable, inspectable, and secure.

**Deliverables:** SQLCipher-compatible SQLite, profile lock and one writer,
forward migrations, encrypted recovery backup, workspace CRUD, global host
references, scoped settings, versioned redacted export, OS credential-store
integration, opaque credential handles, retention settings, and plaintext
canary tests.

**Dependencies:** Phase 1 and stable session identifiers from Phase 2.

**Primary risks:** database packaging, key loss, unavailable credential stores,
migration defects, data corruption, and accidental secret exposure.

**Definition of done:** settings and workspaces survive restart; a second writer
is excluded; migration, recovery, and credential-store denial cases pass; and
protected canaries do not appear in database side files, temporary files,
backups, exports, IPC, or logs.

## Phase 4 — Host management and SSH

**Goal:** Connect reliably and transparently to remote hosts.

**Deliverables:** host profiles, groups, tags, favorites, OpenSSH diagnosis,
safe-subset SSH configuration parsing, protected generated configuration,
Relio-managed known-host review, one-time askpass helper, agent/key selection,
jump-host support, interactive SSH sessions, and typed diagnostics.

**Dependencies:** Phases 2–3.

**Primary risks:** OpenSSH/platform differences, executable configuration
directives, helper spoofing, authentication edge cases, unsafe host-key
behavior, and subprocess leaks.

**Definition of done:** users can create or import a host, review identity
evidence, connect, and receive actionable errors without exposing secrets.
Unknown, changed, and revoked keys; hostile arguments; jump failures; helper
abuse; cancellation; and process cleanup pass negative tests.

## Phase 5 — SFTP, SCP, remote browser, and editing

**Goal:** Make remote file operations first-class and safe.

**Deliverables:** structured file-operation contract, bounded SFTP binary
protocol over a separate supervised OpenSSH subsystem, remote directory
listing, SFTP upload/download, SCP workflow only when diagnosed SFTP semantics
are available, explicit refusal of legacy SCP, bounded progress and
cancellation, permissions and symlink display, remote file browser, bounded
built-in UTF-8 text editor, memory-only unsaved buffers, conflict-aware save,
and atomic replacement where available.

**Dependencies:** Phase 4.

**Primary risks:** malformed or oversized SFTP packets, request correlation,
partial transfers, unsafe path interpretation, accidental legacy-protocol
activation by an external executable, non-text paths, permission errors,
accidental overwrite, symlink races, sensitive editor buffers, and filesystems
without atomic rename.

**Definition of done:** users can browse, transfer, and edit remote files with
visible host and path context. Legacy SCP is unavailable. Tests cover
hostile filenames and packets, interrupted transfer recovery, request-ID
confusion, overwrite confirmation, conflicts, metadata limits, indeterminate
external-command progress, editor size/encoding refusal, plaintext-draft
absence, and best-effort memory cleanup on editor close.

## Phase 6 — Port forwarding

**Goal:** Replace error-prone tunnel command composition with a clear manager.

**Deliverables:** local, remote, and dynamic forwarding models; create, edit,
stop, and restart controls; loopback default; broad-bind confirmation; owned
listener and control-socket supervision; status indicators; conflict
diagnostics; and workspace association.

**Dependencies:** Phase 4.

**Primary risks:** accidental network exposure, orphaned listeners, reconnect
duplication, and privileged ports.

**Definition of done:** users can create a tunnel, see both endpoints and bind
scope, verify state, and stop the owned listener. Reconnect cannot duplicate a
listener, and Relio never kills an unrelated process merely because it uses the
same port.

## Phase 7 — History, snippets, search, logging, and recording

**Goal:** Improve recall and repeated command workflows without hiding shell
truth or retaining data unexpectedly.

**Deliverables:** single-line command snippets, bounded parameter prompts,
reviewed insertion without synthetic Enter, opt-in derived history, opt-in
encrypted segmented recording, log viewer, local search indexes,
command-palette completion, retention and free-space controls, sensitive-output
warnings, and deletion workflows.

**Dependencies:** Phases 2–6.

**Primary risks:** secrets in retained output or indexes, recording corruption,
disk exhaustion, ambiguous command boundaries, and slow search.

**Definition of done:** users can find retained data and reuse a reviewed
snippet without automatic submission while understanding what is stored,
encrypted, indexed, retained, exportable, and deletable. Recording and derived
history remain off until enabled, and control-character, no-synthetic-Enter,
bounded-queue, retention, deletion, and privacy tests pass.

## Phase 8 — Theme engine, customization, and accessibility

**Goal:** Deliver a polished, modern, customizable, keyboard-first workbench.

**Deliverables:** bundled presets, user-created local themes, semantic UI and
terminal tokens, schema validation, local theme editing/reset, shortcut
editing, responsive layouts, reduced motion, screen-reader coverage, WCAG 2.2
AA contrast checks, hostile theme fixtures, and visual regression tests.

**Dependencies:** Phases 1–7.

**Primary risks:** unreadable themes, spoofed safety chrome, platform font and
input differences, shortcut conflicts, and rendering regressions.

**Definition of done:** users can customize appearance and keyboard behavior
without loading scripts, arbitrary styles, fonts, or remote assets. Reserved
safety surfaces remain identifiable and accessible under every valid theme.

## Phase 9 — Hardening and stable release

**Goal:** Make Relio trustworthy to install, operate, update, and recover.

**Deliverables:** previewed crash diagnostics, measured performance budgets,
accessibility audit, threat-model review, independent security review, signed
builds, checksums, software bill of materials, provenance, platform installers,
signed updates, signing-key recovery exercise, migration and rollback tests,
release notes, privacy review, and support matrix.

**Dependencies:** all v1 feature phases.

**Primary risks:** signing credential compromise, update replay, platform
packaging failures, encrypted-data upgrade regressions, and support burden.

**Definition of done:** a clean Tier 1 machine can verify, install, launch,
upgrade, recover from a failed migration, roll back through the documented
data-aware flow, and uninstall a signed build. Security, privacy,
accessibility, performance, migration, and platform gates have evidence and no
unresolved release blocker.

## Maintenance gate

Each phase assigns an owning area, dependency inventory changes, documentation
owner, support impact, and removal/rollback strategy. A milestone is not done if
its tests, fixtures, platform behavior, or runbook have no maintainer. Phase 0
also establishes the ownership map and governance policy; Phase 9 rehearses
maintainer departure and release/security role recovery.

## Scope control

Each phase delivers a narrow usable slice. A proposed capability must appear in
the v1 feature map, fit the current trusted architecture, and include security,
privacy, platform, performance, migration, and maintenance impact. Otherwise it
requires an explicit scope decision and cannot enter a milestone implicitly.

Concepts parked in [future ideas](future-ideas.md) create no v1 package, API,
service, schema, permission, dependency, or compatibility requirement.
