# Implementation Log

This log records milestone outcomes, assumptions, verification, and deferred
work. The [master tracker](MILESTONES.md) remains authoritative for status.

## 2026-07-29 — Milestone 01: Project Governance and Release Prerequisites

### Outcome

Completed using the repository's autonomous-execution defaults.

### Decisions

- License: MIT.
- Copyright: `Copyright (c) 2026 Project Contributors`.
- Default branch: `main`.
- Placeholder repository, area, release, security, and incident owner:
  `@owner`.
- Placeholder private security contact: `security@localhost.invalid`.
- Reference platforms: Ubuntu 24.04 LTS, Windows 11, and macOS Sonoma.
- Existing governance, dependency adoption, ADR, review, and release policies
  remain authoritative.

### Placeholders

- TODO: Replace `@owner` with the real GitHub organization and teams.
- TODO: Replace `security@localhost.invalid` with a monitored private address.
- TODO: Record physical reference-machine specifications and maintainers.
- TODO: Add independent backup owners for security, release, and signing before
  stable release.

### Verification

- Documentation and ownership coverage were reviewed locally.
- Private vulnerability-reporting delivery test: **Deferred** because
  `security@localhost.invalid` is intentionally non-deliverable.
- Hosted branch-protection/CODEOWNERS enforcement: **Deferred** until the
  repository hosting owner replaces the placeholder.
- No application tests apply to this governance-only milestone.

## 2026-07-29 — Milestone 02: Repository and Toolchain Bootstrap

### Outcome

Completed the repository-only bootstrap without implementing the Milestone 03 application shell.

### Decisions

- Pinned Rust 1.97.1, Node.js 24.18.0 LTS, pnpm 11.15.1, and Tauri CLI 2.11.4.
- Added one Cargo workspace, one pnpm workspace, reviewed lockfile, read-only CI, and dependency-free repository checks.
- Application directories remain documentation-only until their milestone.

### Verification

- Documentation formatting, links, fences, JavaScript syntax, pnpm lockfile, lint, and repository checks: **Passed locally**.
- Local Node.js 22.22.2 differs from pinned Node.js 24.18.0: **Deferred to CI**.
- Rust formatting, Clippy, metadata, and Cargo lockfile: **Deferred** because Rust is unavailable locally and no Rust package exists yet.
- Tier 1 clean-machine and hosted security scans: **Deferred** until hosted CI runs.

### Follow-up

- Generate and review `Cargo.lock` when Milestone 03 adds the first Rust package.
- Replace deferred verification with CI links when the repository is hosted.


## 2026-07-29 — Milestone 03: Secure Desktop Application Shell

### Outcome

Completed the minimal Tauri and React composition root without introducing
product-specific privileges, persistence, remote content, or startup network
activity.

### Decisions

- The production webview accepts bundled content only and uses a restrictive CSP.
- The primary window declares one explicit capability with an empty permission
  list; the Rust builder registers no commands or plugins.
- Loading, fatal bootstrap, and unsupported-platform states are local React/HTML
  surfaces. Fatal states provide an explicit safe-exit action.
- The UI error boundary reports only local render diagnostics. A structured,
  redacted logging backend belongs to Milestone 04 and Milestone 32.
- Dependency declaration keeps build tooling in `devDependencies`; runtime
  dependencies are React only.

### Verification

- Strict TypeScript typecheck: **Passed locally**.
- React shell and platform-state tests: **3 passed**.
- CSP, capability, and remote-asset configuration tests: **3 passed**.
- Vite production build with bundled assets: **Passed locally**.
- Repository documentation, lint, and syntax checks: **Passed locally**.
- Native Cargo build and packaged Tier 1 launch: **Deferred** because Rust and
  platform packaging toolchains are unavailable in the current environment.
- Startup network observation is enforced structurally by no network dependency,
  no network capability/plugin, a restrictive CSP, and remote-origin tests;
  packaged process observation is **Deferred to Tier 1 CI**.

### Follow-up

- Generate and review `Cargo.lock` when a Rust-capable environment runs the
  workspace.
- Replace deferred packaged-launch evidence with Tier 1 CI links.


## 2026-07-29 — Milestone 04: Typed IPC, Errors, and Operation Foundation

### Outcome

Completed the product-neutral communication foundation without exposing a generic
privileged Tauri command or implementing future feature APIs.

### Decisions

- Rust DTOs are canonical; `ts-rs` emits reviewed TypeScript artifacts and the
  repository check regenerates them in a temporary directory to detect drift.
- Metadata is bounded at 5 MiB, revisions and event/stream sequences use bounded
  32-bit integers that are lossless in JavaScript, and UUIDv7 values remain opaque.
- Public errors contain stable codes and safe parameters only. Diagnostic cause
  chains remain core-owned. Malformed frontend responses become a fixed safe error.
- Operation transitions, cancellation, idempotency, event replay, confirmation
  consumption, and stream credit are independent framework-neutral modules.
- Tauri is feature-gated for platform-neutral core checks. Native desktop builds
  explicitly enable `desktop-runtime`; this avoids requiring webview libraries for
  contract-only tests.
- Mandatory Clippy rules remain denied. Pedantic and nursery groups are opt-in to
  prevent toolchain upgrades from turning subjective new lints into unrelated
  release blockers.

### Verification

- Generated Rust/TypeScript contract freshness: **Passed**.
- Rust formatting and Clippy: **Passed**.
- Rust contract, hostile-input, and property tests: **29 passed**.
- Frontend shell, security, typed-client, safe-error, and event-gap tests: **9 passed**.
- Strict TypeScript typecheck and full repository check: **Passed**.
- Native Tauri runtime build remains **Deferred to Tier 1 CI** because the current
  Linux environment does not provide GTK/WebKit development libraries.

### Security Notes

No credential bytes, generic filesystem/network/shell primitive, or raw
infrastructure error is represented in this contract. The webview still has an
empty Tauri capability; feature-specific commands and permissions are introduced
only by their owning milestones.


## 2026-07-29 — Milestone 05: Design System and Accessibility Baseline

### Outcome

Completed the bundled semantic-token and accessible component baseline for future
Relio feature surfaces.

### Decisions

- Components consume dark/light semantic tokens derived from the normative design
  system; ordinary feature styling no longer depends on palette literals.
- Native controls and dialog semantics provide the baseline before custom behavior.
  Tabs use roving focus and expected arrow/Home/End navigation.
- Trusted confirmation uses a native modal dialog, invariant reserved tokens, exact
  challenge identity, escaped evidence, safe cancellation, and an explicit action.
  Rust remains the authorization boundary.
- Forced colors, increased contrast, reduced motion, wrapping, rem sizing, and the
  720 × 480 minimum content area are structural CSS requirements.
- The deterministic component fixture includes long, bidirectional, non-Latin, and
  markup-shaped hostile content for Tier 1 visual review.

### Verification

- Strict TypeScript typecheck: **Passed**.
- Component semantic, keyboard, loading, hostile-content, and trust tests: **5 passed**.
- Bundled dark/light WCAG AA contrast-pair tests: **6 passed**.
- Complete frontend test suite: **20 passed**.
- Production frontend bundle: **Passed**.
- Tier 1 screenshot baselines and manual screen-reader checks: **Deferred to native
  visual CI**, with review modes and fixtures documented.

### Security Notes

Theme data cannot supply trusted safety tokens or approval state. Remote and
imported strings are rendered as text, and confirmation decisions return the
original core-issued challenge rather than constructing authority in the view.


## 2026-07-29 — Milestone 06: Workbench Navigation and Application Layout

### Outcome

Completed the responsive, keyboard-navigable application workbench and replaced
the bootstrap preview with the first useful local-only application shell.

### Decisions

- The route model contains only the four finalized v1 top-level destinations:
  Workspaces, Hosts, Library, and Settings. Contextual tools remain sidebar items.
- Route transitions intentionally focus the active-view heading; selecting the
  already-active destination preserves the user current focus.
- The activity rail, target/environment top context, and status bar remain stable
  anchors. Sidebar and inspector become explicit Escape-closeable overlays at
  narrow breakpoints.
- Placeholder surfaces state `Preview only` and describe ownership without
  claiming persistence, connection, or infrastructure capabilities.
- Empty, loading, error, and unavailable states share text and non-color symbols.

### Verification

- Route, keyboard, focus restoration, secondary-region, and state tests: **8 passed**.
- Complete frontend test suite: **28 passed**.
- Strict TypeScript, repository checks, Rust checks, and production build: **Passed**.
- Headless 720 × 480 render smoke: **Passed**.
- Tier 1 native screenshot baselines: **Deferred to native visual CI**.

### Security Notes

Navigation labels and routes are static bundled data. Remote, terminal, imported,
and plugin content has no route-registration path and cannot create trusted status
or infrastructure capability claims.


## 2026-07-29 — Milestone 07: Action Registry and Command Palette

### Outcome

Completed the bounded action registry, shortcut resolver, and accessible command palette, and routed every existing workbench mutation through that shared action model.

### Decisions

- The v1 registry accepts static bundled definitions only; there is no runtime registration surface for plugins or remote content.
- Action availability is evaluated against explicit context and returns a user-facing disabled reason. Privileged operations will still be reauthorized by Rust when their owning milestones arrive.
- Registry size, query length, result count, and recent-action memory are bounded. Recent actions remain telemetry-free and process-local.
- Shortcuts are explicit, scoped, conflict-checked, and cannot synthesize terminal input.
- The palette uses native modal semantics, deterministic keyboard navigation, focus restoration, and inert text rendering for action labels.

### Verification

- Action registry, availability, bounds, performance, shortcut conflict, and hostile-label tests: **8 passed**.
- Workbench routing, keyboard, focus, responsive-region, and palette integration tests: **9 passed**.
- Complete frontend test suite: **37 passed**.
- One-thousand-action bounded search performance budget: **Passed**.
- Generated-contract freshness, documentation checks, Rust formatting, Clippy, Rust tests, strict TypeScript, and production frontend build: **Passed**.
- Native Tauri packaging remains **Deferred to Tier 1 CI** because GTK/WebKit development libraries are unavailable in the current environment.

### Security Notes

Untrusted data cannot register actions, shortcuts, routes, or authority-bearing state. Disabled actions cannot dispatch, action labels are rendered as text, and frontend action dispatch remains intent rather than authorization.


## 2026-07-29 — Milestone 08: Window, Startup, and Shutdown Lifecycle

### Outcome

Completed the deterministic desktop lifecycle foundation: authenticated single-instance forwarding, startup and shutdown state machines, safe window restoration, close review, authority invalidation, and durable unclean-exit detection.

### Decisions

- Primary ownership uses an OS file lock in a user-private runtime directory. A fresh process-lifetime token authenticates a bounded 4 KiB loopback protocol; paths, commands, URLs, unknown fields, non-loopback addresses, and unsupported versions are rejected.
- File locking, rather than file presence, establishes ownership and recovers automatically after process death. Secondary launches can only activate the primary or request an opaque workspace ID.
- Lifecycle metadata is non-sensitive, capped at 4 KiB, written through a user-private temporary file plus sync and atomic rename, and marked unclean before application work begins.
- Restored geometry must intersect a current display and remain within the 720 × 480 minimum and 16,384-pixel hard maximum. Minimized and focus state are never restored.
- Close review and shutdown share one coordinator with ten-second graceful and three-second child escalation budgets. The current shell has no blockers, so review advances immediately; future blocker owners must register typed participants.
- OS lock, unlock, and webview-loss events advance an authority epoch. Unlock never recreates leases or approvals. Platform event adapters terminate at this coordinator rather than changing feature state directly.

### Verification

- Full repository check, generated-contract freshness, documentation checks, Rust formatting, Clippy, TypeScript, and production frontend build: **Passed**.
- Frontend lifecycle and complete frontend suite: **41 passed**.
- Rust unit, hostile-contract, IPC integration, property, lifecycle, persistence, and session-security tests: **44 passed**.
- Authenticated forwarding, spoof rejection, and primary/secondary endpoint tests executed outside the restricted socket sandbox: **Passed**.
- Native Tauri compile and Tier 1 GUI/OS-session conformance: **Deferred to Tier 1 CI** because this host lacks GTK, GLib, and WebKit development packages and does not permit installing them.

### Security Notes

Loopback is treated only as transport. Endpoint authentication derives from user-private metadata, privileged handlers still reauthorize intent, forced termination cannot produce a clean marker, and focus loss is not misclassified as an OS session lock.


## 2026-07-29 — Milestone 09: Local PTY Runtime

### Outcome

Completed the renderer-neutral local terminal runtime with native POSIX PTY and Windows ConPTY adapters, structured shell discovery, bounded ordered streams, resize, exit, cancellation, and owned process-tree cleanup.

### Decisions

- ADR-010 pins `portable-pty 0.9.0` behind Relio-owned adapter traits. Target-specific `nix 0.28.0` and `win32job 2.0.3` provide safe process-group and kill-on-close Job Object containment without project-owned unsafe code.
- Shell programs must be absolute executable files. IDs, argument count and size, working directory, dimensions, and environment are validated before allocation; program and arguments are never concatenated.
- Child environments are cleared and rebuilt from a bounded non-secret platform allowlist. Authentication sockets, tokens, and arbitrary parent-process variables are not inherited.
- Output is opaque hostile bytes and is read only against at most 4 MiB receiver credit, in 64 KiB chunks through a 16-chunk bounded channel. Input is exact-sequence, 64 KiB per frame, 1 MiB pending, and 64 frames maximum.
- Graceful close drops the PTY writer. After three seconds, POSIX kills the owned process group and Windows drops a kill-on-close Job Object. The child waiter emits one exit result and reaps the process.

### Verification

- Full repository check, generated contracts, documentation, Rust formatting, Clippy, TypeScript, and production frontend build: **Passed**.
- Complete frontend suite: **41 passed**.
- Rust unit, hostile-contract, IPC integration, property, lifecycle, PTY fake, and native Linux PTY tests: **53 passed**.
- Native Linux PTY start, output, ordered input, resize, normal exit, forced descendant cleanup, and unrelated-sibling survival: **Passed**.
- Windows x86_64 cross-compile including ConPTY and Job Object paths: **Passed**.
- Windows 11 execution and macOS Sonoma native PTY execution: **Deferred to Tier 1 CI**. The temporary macOS target download timed out; implementation remains covered by the POSIX adapter and release conformance gate.

### Security Notes

The webview receives no process handle or shell primitive. Terminal output stays untrusted bytes, environment inheritance excludes common secret-bearing variables, queue growth is bounded, and termination targets only the PTY process group or Job Object created for the session. A POSIX descendant that deliberately creates a new session is documented as residual platform risk and remains part of release conformance.


## 2026-07-29 — Milestone 10: Terminal Rendering

### Outcome

Completed the first usable local terminal surface with a renderer-neutral, bounded Tauri channel contract and an xterm.js model that survives DOM detachment without routing terminal bytes through React state.

### Decisions

- ADR-011 pins xterm.js 6.0.0 and the fit addon 0.11.0. Clipboard, web-link, image, serialization, WebGL, and experimental Unicode addons remain excluded.
- Rust owns the PTY and grants the main webview exactly six typed local-terminal commands. One active session is an explicit temporary M10 limit replaced by the M11 registry.
- A 1 MiB initial credit window is replenished only after xterm write acknowledgement; sequence counters cross IPC as decimal strings and gaps render in trusted chrome.
- OSC 52, remote window operations, automatic URI opening, and credential-bearing or non-HTTP links are blocked. Multiline or control-bearing paste requires exact-text review.
- The terminal feature is lazy-loaded, reducing the empty-workbench production JavaScript chunk from 547 kB to 205 kB.

### Verification

- Generated IPC contract freshness, documentation checks, Rust formatting, Clippy, strict TypeScript, and production frontend build: **Passed**.
- Complete frontend suite: **47 passed**, including hostile title, URI, clipboard, credit, gap, and a 32 MiB sustained adapter gate (under 2 seconds).
- Rust unit, hostile-contract, IPC, property, lifecycle, and native Linux PTY tests: **62 passed**.
- Windows desktop-runtime feature compilation: **Deferred to native packaging CI** because the repository intentionally has no Windows icon before Milestone 37; the build script stops at that packaging prerequisite. Linux native Tauri compile remains unavailable because this host lacks GTK/WebKit development packages.
- Tier 1 xterm paint, IME, Unicode-font, screen-reader, and physical latency/memory measurements remain release gates in Milestones 34–36.

### Security Notes

Terminal output remains opaque hostile bytes. The renderer has no generic process, filesystem, network, opener, or Tauri clipboard authority; remote text can modify only the bounded terminal model or inert, sanitized presentation facts.
