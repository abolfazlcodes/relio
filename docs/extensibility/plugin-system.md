# Plugin System

## Purpose

Plugins allow the community to add infrastructure integrations and workflows without turning the core into a collection of vendor-specific modules. The plugin system is a product boundary and a security boundary.

## Design principles

- plugins are optional; core workflows work without a marketplace;
- plugins are isolated from the main process;
- capabilities are explicit, least-privilege, and user-visible;
- contributions are declarative where possible;
- plugin APIs are versioned and tested against compatibility fixtures;
- a broken plugin should be disableable without corrupting the workspace;
- first-party integrations use the public contract when practical.

## Plugin lifecycle

```mermaid
flowchart LR
    DISCOVER[Discover manifest]
    VERIFY[Validate version and integrity]
    CONSENT[Show requested capabilities]
    INSTALL[Install in plugin directory]
    ACTIVATE[Lazy activation]
    RUN[Out-of-process host]
    STOP[Stop and revoke resources]

    DISCOVER --> VERIFY --> CONSENT --> INSTALL --> ACTIVATE --> RUN --> STOP
```

Activation should happen on demand or when a declared event occurs. Startup must not launch every installed plugin.

## Manifest concept

A manifest should declare:

- stable plugin ID and publisher namespace;
- display name, description, version, and license;
- supported application and protocol versions;
- entrypoint and runtime type;
- contributions;
- requested capabilities;
- settings schema and storage namespace;
- compatibility and migration information.

The manifest is metadata, not executable permission. Runtime calls are checked against the grants that the user approved.

## Contribution points

The first public contribution points should remain small:

- commands and command-palette entries;
- context-menu actions;
- workspace and host detectors;
- read-only status cards and tool views;
- snippets and workflow templates;
- theme packages;
- infrastructure providers with a defined operation contract.

Later contribution points may include file-system providers, log sources, terminal decorations, and authentication providers. Each requires a separate threat and lifecycle review.

## Capability examples

Capabilities are named around actions rather than blanket access:

- `workspace.read`
- `workspace.write`
- `host.metadata.read`
- `session.observe`
- `session.input.request`
- `filesystem.read(user-selected-path)`
- `filesystem.write(user-selected-path)`
- `network.connect(host-profile)`
- `network.listen(local-port)`
- `secrets.reference.read`

No plugin gets raw credential bytes, arbitrary process execution, unrestricted filesystem access, or arbitrary UI DOM access by default.

## Runtime contract

The initial runtime uses versioned JSON-RPC over a process boundary. Messages have request IDs, typed parameters, typed errors, deadlines, and cancellation. The host enforces message size, call duration, output, and process resource limits where the platform permits.

Plugins should be able to be authored in more than one language. The SDK should provide TypeScript first and document the protocol so other language bindings can follow.

## Failure and compatibility

- plugin crashes are contained and reported with the plugin ID;
- plugin migrations are explicit and reversible where practical;
- unsupported API versions prevent activation with a useful message;
- disabling a plugin removes its active contributions but preserves user data in its namespace;
- plugin updates do not silently widen permissions;
- built-in and external plugins are distinguishable in the UI.

## Marketplace boundary

The marketplace is an optional catalog and distribution service, not the plugin runtime. Local installation from a package or path must remain possible. A marketplace may provide signatures, reviews, compatibility metadata, and update notifications, but installation always shows publisher, permissions, version, and integrity information.

## AI and plugins

An AI plugin may explain a command, generate a draft, summarize approved logs, or suggest a troubleshooting path. It must not have implicit permission to type into a terminal, read secrets, or execute commands. Any execution path is a separate, visible capability and confirmation flow.
