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
