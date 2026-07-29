# Developer Operations Workspace

> Project: Relio

An open-source, local-first desktop workspace for operating development and infrastructure environments.

Relio is intended to feel like an IDE for infrastructure: a fast terminal at its center, surrounded by host management, workspaces, file operations, port forwarding, observability, automation, and carefully bounded extensibility.

The project is currently in the documentation and architecture phase. There is no production application code yet.

## Product direction

The application combines the engineering depth of established terminal and SSH clients with the discoverability and extensibility of a modern IDE. It is not a clone of any existing product. We study proven workflows, then design a coherent local-first experience around infrastructure work.

The first release should be useful without an account, a cloud service, or an AI provider. Synchronization, collaboration, and AI are optional layers.

## Principles

- Local-first by default.
- Fast startup and low idle cost.
- Terminal compatibility before visual novelty.
- Explicit actions and safe defaults for infrastructure work.
- Small core, stable extension contracts.
- Accessible, keyboard-friendly, customizable UI.
- Cross-platform behavior with platform-appropriate integration.
- Documentation and tests are part of every feature.

## Planned capabilities

Terminal sessions, tabs, panes, session restore, SSH, SFTP, host management, infrastructure workspaces, command snippets and history, session recording, searchable logs, visual port forwarding, remote file editing, themes, plugins, optional synchronization, and an optional AI assistant.

See the [feature map](docs/product/feature-map.md) for scope and sequencing.

## Documentation

Start with the [documentation index](docs/README.md). The most important decisions are:

- [Vision and product principles](docs/product/vision.md)
- [Architecture overview](docs/architecture/overview.md)
- [Technology decisions](docs/architecture/technology-decisions.md)
- [Development roadmap](docs/roadmap.md)
- [Plugin system](docs/extensibility/plugin-system.md)
- [Security architecture](docs/security/README.md)
- [Contributor guide](docs/development/contributing.md)

## Project status

| Area | Status |
| --- | --- |
| Product definition | Documented |
| Architecture | Initial decision set documented |
| Application shell | Not started |
| Terminal runtime | Not started |
| SSH and host management | Not started |
| Plugin and theme SDK | Design only |
| Releases | Not started |

## Contributing

Contributions are welcome once implementation begins. Please read the [developer onboarding guide](docs/development/developer-onboarding.md), [contributing guide](docs/development/contributing.md), and [coding standards](docs/development/coding-standards.md) before opening a pull request.

## License

The project license has not been selected yet. This is an intentional project-governance decision and must be resolved before the first public source release. See the [release strategy](docs/operations/release-strategy.md).
