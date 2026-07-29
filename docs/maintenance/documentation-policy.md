# Documentation Lifecycle

## Sources of truth

Relio avoids multiple documents claiming authority over the same decision:

| Information | Canonical location |
| --- | --- |
| Product purpose and v1 scope | Product vision, feature map, and future ideas |
| Final user interaction | Design specifications |
| Normative component/contracts/lifecycles | Implementation architecture |
| Focused subsystem rationale | Architecture documents |
| Historical architectural decisions | ADRs |
| Security invariants and risks | Security documents and threat model |
| Strategic phases | Roadmap |
| Implementation order, status, and completion evidence | Master development tracker |
| Delivery gates | Operations documents |
| Maintainer process | Maintenance documents |

Focused documents link to a canonical contract instead of restating it.
Intentional summaries identify themselves as summaries. If two normative
documents conflict, use the precedence in the implementation-readiness review
and repair the conflict in the same change that discovers it.

## Decomposition rule

Split a large document when sections have different owners, review triggers, or
release cadence—not merely because of line count. Keep a short canonical index
at the old path, preserve stable anchors or redirects where tooling allows, and
move each normative rule to exactly one child document. The implementation
architecture may be decomposed this way after module owners exist; until then,
its numbered end-to-end contract is intentionally one review unit.

## Required metadata

Once implementation begins, every normative architecture, security, operations,
and maintenance document carries:

- status: proposed, accepted, superseded, or historical;
- accountable owner or owning area;
- last reviewed date;
- review triggers.

This can be front matter or a consistent visible header. Do not add metadata to
every tutorial or transient planning note.

## Change rules

A behavior change updates its contract, UX, tests, migration/release notes, and
security analysis together where applicable. Documentation-only changes still
receive review from the owning area when they alter normative meaning.

Use:

- ADR for a durable decision and alternatives;
- architecture document for current system truth;
- roadmap for sequencing;
- issue for bounded work;
- release notes for user-visible change;
- runbook for repeatable operations;
- comment for local reasoning that cannot be clear from code.

Do not use an issue or pull-request discussion as the only durable record.

## Review schedule

- Security and support policy: each stable release and after an incident.
- Implementation architecture and dependency decisions: every minor release.
- Platform matrix, onboarding, build, and release instructions: quarterly.
- Roadmap and feature map: monthly while active development is underway.
- Competitor analysis and design rationale: yearly or when product direction
  materially changes.

Automated checks verify internal links, duplicate headings where tooling
depends on anchors, Mermaid syntax, spelling of canonical product terms, and
references to renamed/deleted files. Human review checks meaning and freshness.

## Naming glossary

Canonical terms:

- **Relio**: product and repository;
- **workbench**: the complete primary application UI;
- **workspace**: a persisted local composition of references and layout;
- **session**: one live or restorable terminal interaction;
- **pane**: one visual layout leaf;
- **host profile**: reusable remote connection metadata;
- **operation**: one tracked external or long-running action;
- **core**: trusted Rust application authority;
- **webview**: less-trusted React runtime;
- **profile**: encrypted local Relio data boundary.

Avoid “project” for a workspace, “connection” for a session, “backend server”
for the Rust core, and “plugin” for compiled core features. Identifiers use
`snake_case` in Rust persistence/internal fields where idiomatic, `camelCase`
only in generated TypeScript projections, and dotted lowercase names for IPC
operations and error codes.

## Archiving and deletion

Accepted ADRs are never deleted; supersede them. Remove stale tutorials when
they no longer describe a supported release. Historical release runbooks and
compatibility matrices remain available with an explicit version range.
Screenshots are treated as versioned documentation assets and replaced when
they would mislead.
