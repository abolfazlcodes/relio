# Documentation

This directory is the project’s architecture and product record. A feature is not considered ready for implementation until its user value, boundaries, security implications, and test strategy are understandable here.

## Product

- [Vision and product principles](product/vision.md)
- [Feature map](product/feature-map.md)
- [Competitor analysis](product/competitor-analysis.md)

## Product design

- [Complete UX design specification](design/README.md)
- [Product design philosophy](design/product-design-philosophy.md)
- [User personas](design/personas.md)
- [Information architecture](design/information-architecture.md)
- [Core user flows](design/core-user-flows.md)
- [Application layout](design/application-layout.md)
- [Design system](design/design-system.md)
- [Theme system UX](design/theme-system-ux.md)
- [Keyboard-first experience](design/keyboard-first-experience.md)
- [Security UX](design/security-ux.md)
- [Low-fidelity wireframes](design/wireframes.md)

## Architecture

- [Implementation readiness review](implementation-readiness-review.md)
- [Normative implementation architecture and lifecycle contracts](architecture/implementation-architecture.md)
- [Technical blueprint and architecture review](architecture/technical-blueprint.md)
- [Architecture overview](architecture/overview.md)
- [IPC and process model](architecture/ipc-and-process-model.md)
- [Technology decisions](architecture/technology-decisions.md)
- [Architecture decision record index](architecture/adr/README.md)
- [Repository structure](architecture/repository-structure.md)
- [Workspace architecture](architecture/workspaces.md)
- [Persistence architecture](architecture/persistence.md)
- [UI architecture](architecture/ui.md)
- [State management](architecture/state-management.md)
- [Settings system](architecture/settings-system.md)
- [Terminal architecture](architecture/terminal.md)
- [SSH architecture](architecture/ssh.md)
- [Theme system](architecture/theme-system.md)
- [Performance and capacity](architecture/performance-and-capacity.md)
- [Platform support](architecture/platform-support.md)

## Security

- [Security architecture index](security/README.md)
- [Threat model](security/threat-model.md)
- [Credential storage](security/credentials.md)
- [Secret management](security/secrets.md)
- [SSH security](security/ssh.md)
- [Local database security](security/local-data.md)
- [Encryption strategy](security/encryption.md)
- [Network security](security/network.md)
- [Update security](security/updates.md)
- [Supply-chain security](security/supply-chain.md)
- [Secure development lifecycle and disclosure](security/secure-development.md)
- [Privacy principles](security/privacy.md)

## Maintenance

- [Maintenance index](maintenance/README.md)
- [Ten-year maintainability review](maintenance/maintainability-review.md)
- [Governance and ownership](maintenance/governance.md)
- [Dependency lifecycle policy](maintenance/dependency-policy.md)
- [Documentation lifecycle](maintenance/documentation-policy.md)
- [Compatibility and support policy](maintenance/compatibility-policy.md)

## Delivery and operations

- [Development roadmap](roadmap.md)
- [Testing strategy](operations/testing-strategy.md)
- [Security considerations](operations/security.md)
- [Release strategy](operations/release-strategy.md)
- [Versioning strategy](operations/versioning.md)
- [Future ideas](future-ideas.md)

## Contribution

- [Master development tracker](development/MILESTONES.md)
- [Developer onboarding](development/developer-onboarding.md)
- [Contributing guide](development/contributing.md)
- [Coding standards](development/coding-standards.md)
- [Pull request template](../.github/pull_request_template.md)

## How to use this documentation

Architecture documents describe stable boundaries and constraints. The roadmap
describes strategic phases; the master development tracker owns implementation
order, status, checklists, and completion evidence. If implementation reveals a meaningful change in a boundary or tradeoff, update the relevant document and add an architecture decision record before or alongside the code change.
