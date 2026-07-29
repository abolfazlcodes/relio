# Developer Operations Workspace

> Project: Relio

An open-source, local-first desktop workspace for operating development and infrastructure environments.

Relio is intended to feel like an IDE for remote operations: a fast terminal at
its center, surrounded by host management, local workspaces, secure file
operations, port forwarding, search, and session observability.

The project is currently in the documentation and architecture phase. There is no production application code yet.

## Product direction

The application combines the engineering depth of established terminal and SSH
clients with the discoverability and polish of a modern desktop workbench. It is
not a clone of any existing product. We study proven workflows, then design a
coherent local-first experience around remote operations.

Relio v1 has no account requirement or hosted service layer. Its feature set and
data remain local except for connections and updates the user explicitly
requests.

## Principles

- Local-first by default.
- Fast startup and low idle cost.
- Terminal compatibility before visual novelty.
- Explicit actions and safe defaults for infrastructure work.
- Small trusted core with no runtime-loaded application code.
- Accessible, keyboard-friendly, customizable UI.
- Cross-platform behavior with platform-appropriate integration.
- Documentation and tests are part of every feature.

## Planned capabilities

Terminal sessions, tabs, split panes, session restore, SSH, SFTP, SCP, host and
workspace management, command snippets and history, command palette, local
search, logging, session recording, visual port forwarding, remote file
browsing/editing, and a built-in theme engine.

See the [feature map](docs/product/feature-map.md) for scope and sequencing.

## Documentation

Start with the [documentation index](docs/README.md). The most important decisions are:

- [Vision and product principles](docs/product/vision.md)
- [Implementation-ready technical blueprint](docs/architecture/technical-blueprint.md)
- [Architecture overview](docs/architecture/overview.md)
- [Technology decisions](docs/architecture/technology-decisions.md)
- [Development roadmap](docs/roadmap.md)
- [Theme system](docs/architecture/theme-system.md)
- [Security architecture](docs/security/README.md)
- [Contributor guide](docs/development/contributing.md)

## Project status

| Area | Status |
| --- | --- |
| Product definition | Documented |
| Architecture | Reviewed implementation baseline documented |
| Application shell | Not started |
| Terminal runtime | Not started |
| SSH and host management | Not started |
| Theme engine | Architecture documented |
| Releases | Not started |

## Contributing

Contributions are welcome once implementation begins. Please read the [developer onboarding guide](docs/development/developer-onboarding.md), [contributing guide](docs/development/contributing.md), and [coding standards](docs/development/coding-standards.md) before opening a pull request.

## License

The project license has not been selected yet. This blocks dependency
distribution decisions and the first public source or binary release. It must
be resolved before implementation dependencies are accepted. See the
[technical blueprint](docs/architecture/technical-blueprint.md) and
[release strategy](docs/operations/release-strategy.md).
