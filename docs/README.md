# Documentation

This directory is the project’s architecture and product record. A feature is not considered ready for implementation until its user value, boundaries, security implications, and test strategy are understandable here.

## Product

- [Vision and product principles](product/vision.md)
- [Feature map](product/feature-map.md)
- [Competitor analysis](product/competitor-analysis.md)

## Architecture

- [Architecture overview](architecture/overview.md)
- [Technology decisions](architecture/technology-decisions.md)
- [Repository structure](architecture/repository-structure.md)
- [UI architecture](architecture/ui.md)
- [State management](architecture/state-management.md)
- [Settings system](architecture/settings-system.md)
- [Terminal architecture](architecture/terminal.md)
- [SSH architecture](architecture/ssh.md)

## Extensibility

- [Plugin system](extensibility/plugin-system.md)
- [Theme system](extensibility/theme-system.md)

## Security

- [Security architecture index](security/README.md)
- [Threat model](security/threat-model.md)
- [Credential security](security/credentials.md)
- [SSH security](security/ssh.md)
- [Plugin security](security/plugins.md)
- [Local data security](security/local-data.md)
- [Network security](security/network.md)
- [Supply-chain security](security/supply-chain.md)
- [Secure development and disclosure](security/secure-development.md)

## Delivery and operations

- [Development roadmap](roadmap.md)
- [Testing strategy](operations/testing-strategy.md)
- [Security considerations](operations/security.md)
- [Release strategy](operations/release-strategy.md)
- [Versioning strategy](operations/versioning.md)

## Contribution

- [Developer onboarding](development/developer-onboarding.md)
- [Contributing guide](development/contributing.md)
- [Coding standards](development/coding-standards.md)
- [Pull request template](../.github/pull_request_template.md)

## How to use this documentation

Architecture documents describe stable boundaries and constraints. Roadmap documents describe sequencing and may change. If implementation reveals a meaningful change in a boundary or tradeoff, update the relevant document and add an architecture decision record before or alongside the code change.
