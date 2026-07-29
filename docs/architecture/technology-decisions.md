# Technology Decisions

These decisions describe the intended foundation, not a frozen dependency lockfile. Revisit a decision when new evidence changes the product constraints, not merely because another technology is fashionable.

## Decision summary

| Area | Decision | Status |
| --- | --- | --- |
| Desktop shell | Tauri 2 | Accepted |
| Native core | Rust | Accepted |
| Frontend | React + TypeScript | Accepted |
| Frontend tooling | Vite | Accepted for initial implementation |
| JavaScript workspace | pnpm workspace, pinned in `packageManager`, with one lockfile | Accepted |
| Rust async runtime | Tokio through a small application-owned task boundary | Accepted direction |
| Terminal rendering | xterm.js | Accepted |
| Local PTY | `portable-pty`-style Rust adapter | Accepted direction |
| SSH transport | Capability interface; supported OpenSSH adapter first | Accepted direction |
| SFTP transport | Bounded SFTP protocol over a supervised OpenSSH subsystem | Accepted direction |
| Persistence | SQLCipher-compatible SQLite behind one writer service | Accepted |
| Secret storage | Native OS credential store | Accepted |
| UI state | Local component state plus small event-driven client store | Accepted |
| Desktop IPC | Tauri commands/events plus bounded binary streams; generated types | Accepted |
| Theme model | Declarative design tokens and semantic roles | Accepted |

## ADR-001: Tauri 2 for the desktop shell

**Context:** The UI needs modern web tooling; the application needs native process, filesystem, credential, and window access. Electron is familiar but bundles a browser runtime and a Node.js process model with a larger baseline footprint.

**Decision:** Use Tauri 2 with the system webview and a Rust host.

**Why:** Tauri documents a small application-specific bundle and a native-webview model, while still allowing any frontend framework and Rust-backed commands. This supports the frontend transition path without making Node.js a production runtime dependency.

**Tradeoffs:** Webview behavior differs between Windows, Linux, and macOS. We
must test visual and input behavior on each platform. Trusted platform adapters
may still be needed in the Rust core for platform-specific features.

**Alternatives considered:** Electron for ecosystem maturity; fully native UI for maximum platform fidelity; a Rust-native UI toolkit for one-language simplicity. Electron was rejected for baseline resource cost and a larger trusted runtime. Fully native and Rust-native UI were rejected for the slower iteration and smaller contributor pool during the first product phase.

Reference: [Tauri: What is Tauri?](https://v2.tauri.app/start/)

## ADR-002: React and TypeScript for the workbench

**Context:** The product is UI-heavy, requires a coherent design system, and
will need contributors to build complex views.

**Decision:** Use React and TypeScript, compiled with Vite, with a project-owned design system.

**Why:** The project can iterate quickly, TypeScript makes IPC and feature
contracts explicit, and React has a broad contributor and testing ecosystem.

**Tradeoffs:** The frontend must not become a second application core. We will enforce a small IPC client surface, avoid arbitrary cross-component global state, and profile rendering with realistic terminal output.

**Alternatives considered:** Svelte for lower ceremony; a native Rust UI for fewer language boundaries; Vue for a different component model. None has a decisive advantage over the expected contributor and UI complexity profile.

## ADR-003: xterm.js as the first terminal renderer

**Context:** Terminal compatibility is critical, but rendering a VT terminal from scratch is a specialized multi-year effort.

**Decision:** Use xterm.js for the initial terminal frontend and keep the backend stream contract renderer-neutral.

**Why:** xterm.js exposes terminal buffers, parser hooks, link handling, Unicode behavior, and an addon model. It lets us focus early engineering on PTY lifecycle, transport reliability, session UX, and performance instead of rebuilding terminal emulation.

**Tradeoffs:** The renderer runs in the webview and must be tuned for large output. Some advanced terminal protocols may require addons or a later native renderer. The backend must not depend on xterm.js-specific data structures.

Reference: [xterm.js documentation](https://xtermjs.org/docs/)

## ADR-004: adapter-based transport with OpenSSH compatibility first

**Context:** SSH interoperability includes config files, agents, keys, jump hosts, host-key verification, PTY allocation, SFTP, and forwarding. A native implementation can offer tight integration but takes substantial time to harden.

**Decision:** Define a transport contract first. Start with a supported
OpenSSH-compatible adapter for interactive SSH, forwarding, authentication, and
host identity. Use a separate OpenSSH SFTP subsystem connection with a bounded
binary protocol client for file operations; do not parse human-readable command
output. Evaluate a native SSH adapter only after measured requirements justify
another protocol stack.

**Why:** Users already depend on OpenSSH behavior, agents, known-hosts files,
jump hosts, and platform-specific helpers. Reusing the supported executable
reduces protocol surprises while a safe-subset parser and generated config keep
executable directives from running implicitly.

**Tradeoffs:** Subprocess adapters require supervision and careful stream
handling. The SFTP client adds a binary protocol parser and a second connection,
but avoids duplicating SSH cryptography and host-key logic. Native SSH can be
introduced behind the same interface only when its benefits justify the
maintenance and security cost.

**Alternatives considered:** A pure Rust SSH implementation from day one; libssh2 bindings; embedding a second SSH client. These remain evaluation options, not first-phase commitments.

## ADR-005: SQLite for local metadata

**Context:** The app needs durable local data, migrations, search, relationships, and offline operation without a server.

**Decision:** Use a SQLCipher-compatible SQLite build behind repositories,
migration boundaries, and one database writer service. Keep the random profile
key in the OS secret store. Do not expose SQL or database schemas to the
frontend.

**Why:** SQLite is mature, portable, transactional, embeddable, and well-suited to a single-user desktop application. It supports workspace relationships and local search without introducing a local server.

**Tradeoffs:** Encryption packaging, attribution, key loss, and cross-platform
builds become release concerns. Concurrent writes need discipline. Large
terminal recordings use encrypted segmented files with metadata in SQLite
rather than turning the database into an unbounded event log.

See [ADR-007](adr/007-encryption-at-rest.md) and
[persistence architecture](persistence.md).

## ADR-006: no dynamic application-code loading in v1

**Context:** Relio handles credentials and sessions that can reach production
systems. Loading separately distributed executable or UI code at runtime would
expand the trusted computing base, dependency graph, release paths, and
authorization model before the core product has proved its own boundaries.

**Decision:** All v1 application behavior is compiled, reviewed, signed, and
released with Relio. User customization is data-only: settings, snippets,
keybindings, layouts, and bounded theme tokens. The application does not load
arbitrary scripts, native libraries, remote pages, or executable modules.

**Why:** A smaller trusted codebase is easier to audit, test, package, and
support. One signed distribution path also makes incident response and
provenance materially clearer.

**Tradeoffs:** New capabilities require a core change and a Relio release.
External tools can still be launched deliberately by the user, but they receive
no hidden in-process authority or privileged API.

**Alternatives considered:** In-process dynamic modules, unrestricted scripts,
and isolated out-of-process runtimes. Each adds substantial security and
lifecycle machinery without helping the focused v1 workflow.

## Additional accepted decisions

- [ADR-008: workspace persistence model](adr/008-workspace-persistence.md)
- [ADR-009: update trust model](adr/009-update-trust.md)

## Decision maintenance

When a decision changes, record context, alternatives, evidence, migration impact, and the new status. Use the [ADR template](adr/000-template.md) for decisions that change module boundaries or public contracts.

A dependency name or exact tool version is not accepted merely by appearing in
an example. Phase 1 must pin the Rust toolchain, Node version, package manager,
Tauri CLI, and lockfiles. A security-critical dependency such as encrypted
SQLite, terminal renderer, or SSH transport also requires maintenance, license,
platform, and update-response evidence.
