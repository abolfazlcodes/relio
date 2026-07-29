# Repository Structure

The repository should grow around boundaries rather than framework conventions. This is a target shape for the first implementation phases; the directories do not need to be created until they contain work.

```text
.
├── .github/
│   ├── ISSUE_TEMPLATE/
│   └── pull_request_template.md
├── docs/
│   ├── architecture/
│   │   └── adr/
│   ├── development/
│   ├── extensibility/
│   ├── operations/
│   └── product/
├── apps/
│   ├── desktop/              # Tauri shell and frontend entrypoint
│   └── plugin-host/          # Plugin process lifecycle and protocol
├── crates/
│   ├── app-core/             # Use cases, domain models, events
│   ├── session-runtime/      # PTY/session lifecycle and stream routing
│   ├── transports/           # Local and remote transport contracts/adapters
│   ├── persistence/          # SQLite repositories and migrations
│   ├── secrets/              # OS credential-store adapter
│   └── protocol/             # Versioned IPC and plugin contracts
├── packages/
│   ├── ui/                   # Design system and accessible primitives
│   ├── workbench/            # Layout, panels, command palette
│   ├── terminal-view/        # Renderer integration and terminal UX
│   └── sdk/                  # Plugin and theme authoring types
├── plugins/
│   └── built-in/             # First-party integrations using public contracts
├── tests/
│   ├── integration/
│   ├── fixtures/
│   └── performance/
└── README.md
```

## Dependency direction

```text
ui / workbench -> protocol client -> app-core contracts
app-core -> protocol, persistence interfaces, transport interfaces
infrastructure adapters -> app-core interfaces
plugins -> sdk and versioned protocol only
```

The frontend may depend on generated types from `protocol`, but it must not import persistence, OS, or transport implementation code. Built-in integrations should use the same public provider contracts as external integrations where practical.

## Naming and ownership

- one module owns one domain concept or technical responsibility;
- public interfaces use nouns for data and verbs for operations;
- platform-specific code lives behind an adapter named for the platform or capability;
- tests live next to the unit under test for local behavior and in `tests/` for cross-module scenarios;
- documentation for a public module lives beside its contract or in `docs/` when it explains system behavior.
