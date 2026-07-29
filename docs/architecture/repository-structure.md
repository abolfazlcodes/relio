# Repository Structure

## Principle

Start with cohesive modules and extract packages only for a real process,
security, reuse, public-contract, or build boundary. Creating a crate for every
domain noun before implementation would add versioning and dependency overhead
without improving isolation.

The repository remains documentation-only until Phase 1 begins.

## Initial implementation shape

```text
.
├── .github/
│   ├── ISSUE_TEMPLATE/
│   ├── CODEOWNERS              # review-enforced area ownership
│   └── workflows/
├── docs/
│   ├── architecture/
│   │   └── adr/
│   │       └── README.md          # indexed decision history
│   ├── development/
│   ├── maintenance/
│   ├── operations/
│   ├── product/
│   └── security/
├── apps/
│   └── desktop/
│       ├── src/                    # React/TypeScript workbench
│       │   ├── app/
│       │   ├── features/
│       │   ├── platform/
│       │   └── ui/
│       ├── src-tauri/
│       │   ├── capabilities/       # explicit per-window Tauri permissions
│       │   ├── migrations/
│       │   └── src/
│       │       ├── application/
│       │       ├── domain/
│       │       ├── infrastructure/
│       │       └── protocol/
│       └── tests/
├── fixtures/
│   ├── terminal/
│   ├── ssh/
│   ├── ssh-config/
│   └── security/
├── tests/
│   ├── end-to-end/
│   ├── performance/
│   └── packaging/
├── LICENSE                    # selected before implementation dependencies
├── Cargo.toml
└── package.json
```

Backend modules are private by default. Frontend features own views and typed
clients, not infrastructure implementations. Tests close to a module cover
local behavior; root suites cover packaged and cross-boundary behavior.

## Extraction triggers

Extract a Rust crate or TypeScript package only when at least one is true:

- it is a separate executable or process boundary;
- it must prevent a dependency from entering a security-sensitive binary;
- two independently built consumers require it;
- it is a versioned process or data protocol;
- build time or platform conditional compilation materially improves;
- a focused fuzzing or `unsafe` boundary needs isolated ownership.

Expected later extractions are:

```text
crates/
├── session-runtime/          # if PTY/transport reuse or fuzzing justifies it
├── remote-transport/         # if SSH/SFTP/SCP reuse justifies it
└── persistence/              # if encryption/build isolation justifies it
```

These directories are created when their phase begins, not as empty
architecture theater.

## Dependency direction

```text
React views -> generated frontend client -> application commands
application services -> domain models and infrastructure interfaces
infrastructure adapters -> application-owned interfaces
```

- Frontend code cannot import backend persistence, transport, or OS modules.
- Domain modules do not depend on Tauri, React, SQLite, or OpenSSH.
- Infrastructure adapters do not decide confirmation, authorization, retention, or
  safety policy.
- Core adapters use narrow application-owned interfaces where that improves
  compatibility testing, without adding a process boundary unless isolation or
  independent lifecycle requirements justify one.
- Cyclic dependencies are prohibited.

## Toolchain ownership

- Pin Rust with `rust-toolchain.toml` when implementation starts.
- Use one committed Cargo lockfile for shipped binaries.
- Pin one JavaScript package manager in the root `packageManager` field and
  commit its lockfile.
- Generate frontend IPC types in reproducible checks; CI fails on stale
  generated output.
- Keep platform packaging configuration beside the desktop app and release
  policy in `docs/operations`.

## Naming and documentation

- one module owns one domain concept or technical responsibility;
- public interfaces use nouns for data and verbs for operations;
- platform code lives behind a capability adapter named for its platform;
- security-sensitive adapters include a local threat note and negative tests;
- public contract changes update documentation and an ADR in the same change;
- generated files identify their source and regeneration command.
