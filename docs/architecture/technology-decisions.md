# Technology Decisions

These decisions describe the intended foundation, not a frozen dependency lockfile. Revisit a decision when new evidence changes the product constraints, not merely because another technology is fashionable.

## Decision summary

| Area | Decision | Status |
| --- | --- | --- |
| Desktop shell | Tauri 2 | Accepted |
| Native core | Rust | Accepted |
| Frontend | React + TypeScript | Accepted |
| Frontend tooling | Vite | Accepted for initial implementation |
| Terminal rendering | xterm.js | Accepted |
| Local PTY | `portable-pty`-style Rust adapter | Accepted direction |
| SSH transport | Transport interface; OpenSSH-compatible provider first | Accepted direction |
| Persistence | SQLite behind repository interfaces | Accepted |
| Secret storage | Native OS credential store | Accepted |
| UI state | Local component state plus small event-driven client store | Accepted |
| Plugin transport | Versioned JSON-RPC over process boundary | Accepted direction |
| Theme model | Declarative design tokens and semantic roles | Accepted |
| Sync | Optional provider, never a core dependency | Accepted principle |

## ADR-001: Tauri 2 for the desktop shell

**Context:** The UI needs modern web tooling; the application needs native process, filesystem, credential, and window access. Electron is familiar but bundles a browser runtime and a Node.js process model with a larger baseline footprint.

**Decision:** Use Tauri 2 with the system webview and a Rust host.

**Why:** Tauri documents a small application-specific bundle and a native-webview model, while still allowing any frontend framework and Rust-backed commands. This supports the frontend transition path without making Node.js a production runtime dependency.

**Tradeoffs:** Webview behavior differs between Windows, Linux, and macOS. We must test visual and input behavior on each platform. Native plugins may still be needed for platform-specific features.

**Alternatives considered:** Electron for ecosystem maturity; fully native UI for maximum platform fidelity; a Rust-native UI toolkit for one-language simplicity. Electron was rejected for baseline resource cost and a larger trusted runtime. Fully native and Rust-native UI were rejected for the slower iteration and smaller contributor pool during the first product phase.

Reference: [Tauri: What is Tauri?](https://v2.tauri.app/start/)

## ADR-002: React and TypeScript for the workbench

**Context:** The product is UI-heavy, requires a coherent design system, and will need contributors to build views and plugins.

**Decision:** Use React and TypeScript, compiled with Vite, with a project-owned design system.

**Why:** The team can iterate quickly, TypeScript makes IPC and contribution contracts explicit, and React has a broad contributor and testing ecosystem.

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

**Decision:** Define a transport contract first. Start with an OpenSSH-compatible adapter where it provides the best interoperability, then evaluate a native SSH adapter after real compatibility and UX requirements are measured.

**Why:** Users already depend on their SSH configuration, agents, known-hosts files, proxy commands, and platform-specific credential helpers. Reusing established client behavior reduces surprises in the first useful release.

**Tradeoffs:** A subprocess adapter requires process supervision and careful stream handling. It may limit fine-grained control in some features. Native SSH can be introduced behind the same interface when the benefits justify its maintenance and security cost.

**Alternatives considered:** A pure Rust SSH implementation from day one; libssh2 bindings; embedding a second SSH client. These remain evaluation options, not first-phase commitments.

## ADR-005: SQLite for local metadata

**Context:** The app needs durable local data, migrations, search, relationships, and offline operation without a server.

**Decision:** Use SQLite behind repositories and migration boundaries. Do not expose SQL or database schemas to the frontend or plugin API.

**Why:** SQLite is mature, portable, transactional, embeddable, and well-suited to a single-user desktop application. It supports workspace relationships and local search without introducing a local server.

**Tradeoffs:** Concurrent writes need discipline. Large terminal recordings should use a file/blob strategy with metadata in SQLite rather than turning the database into an unbounded event log.

## ADR-006: out-of-process plugins with capability grants

**Context:** Plugins are required for ecosystem growth, but arbitrary in-process code could compromise credentials, terminal sessions, stability, and updateability.

**Decision:** Plugins communicate with a plugin host over a versioned JSON-RPC protocol and receive explicit capabilities. UI extension is declarative or rendered through constrained surfaces; arbitrary DOM access is not part of the contract.

**Why:** A process boundary limits crashes and makes permissions visible. JSON-RPC keeps the initial SDK language-neutral. A future WebAssembly runtime may be added for more constrained plugins, but it is not a prerequisite for the first SDK.

**Tradeoffs:** Process startup and IPC add complexity. We will support lazy activation, timeouts, quotas, and clear lifecycle diagnostics.

## Decision maintenance

When a decision changes, record context, alternatives, evidence, migration impact, and the new status. Use the [ADR template](adr/000-template.md) for decisions that change module boundaries or public contracts.
