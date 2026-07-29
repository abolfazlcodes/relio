# Future Ideas

## Out of Scope for v1

This document is a parking lot for concepts that are deliberately excluded from
Relio v1. Nothing here creates a milestone, package, API, service, data model,
permission, dependency, or compatibility requirement for the current
architecture.

Relio v1 prioritizes SSH, terminal fidelity, SFTP/SCP, host and workspace
management, sessions, remote files, forwarding, local search/history, recording,
themes, performance, and secure credentials. A smaller trusted codebase is an
intentional security decision.

## Plugin and extension ecosystem

Out of Scope for v1:

- plugin system;
- plugin SDK;
- third-party extensions;
- community extensions;
- plugin marketplace;
- any marketplace of installable functionality;
- plugin manager, runtime, permissions, sandbox, extension loader, package
  signing, publishing, or distribution service.

Reconsideration would require clear user demand, a new threat model, a realistic
cross-platform isolation strategy, long-term API compatibility ownership,
supply-chain operations, and evidence that the benefit justifies a much larger
attack surface.

## AI capabilities

Out of Scope for v1:

- AI assistant or chat;
- AI command generation;
- AI log analysis;
- AI troubleshooting;
- AI integrations;
- AI service layer, LLM abstraction, prompt management, model credentials, or
  remote context transmission.

Reconsideration would require a separate privacy architecture, data-flow
inventory, explicit execution boundary, provider lifecycle, cost and outage
model, and evidence that local workflows remain complete without it.

## Connected and collaborative services

Out of Scope for v1:

- cloud synchronization;
- team collaboration;
- shared workspaces;
- shared credentials;
- account-dependent features;
- hosted workspace state, cloud sync services, or remote collaboration
  services.

Reconsideration would require end-to-end encryption, identity and recovery
design, conflict semantics, metadata privacy, revocation, deletion, abuse
handling, service operations, and a separate security review.

## Decision rule

These ideas must not be used to justify abstraction in v1. If one is proposed
after v1, start with a product problem and a new ADR; do not assume the previous
designs or terminology remain valid.
