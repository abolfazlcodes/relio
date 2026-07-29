# ADR-008: Store workspaces as local aggregates with global references

- Status: Accepted
- Date: 2026-07-29
- Owners: Relio maintainers

## Context

The original documentation used “workspace” for layout, project, and host
grouping without defining ownership. It did not say whether a workspace was a
folder, a database, or a credential boundary, or what deletion should do to
referenced global hosts.

## Decision

A workspace is a versioned aggregate in the single local Relio profile
database. It owns composition and layout records and references global host and
credential metadata by stable IDs.

Workspaces do not own local project roots or embed credentials. Export uses a
versioned redacted JSON document and never copies the raw database. V1 does not
ingest that document.

Deleting a workspace removes only workspace-owned records and associations.
Global hosts, credentials, and remote resources require separate explicit
deletion.

## Rationale

This model supports host reuse, clear deletion impact, transactional updates,
and local search without duplicating credentials or introducing a local
filesystem authorization model.

## Alternatives considered

- **One database per workspace:** rejected because reusable global hosts,
  global settings, search, and migrations become more complex without adding a
  security boundary.
- **Workspace equals project directory:** rejected because v1 workspaces are
  remote-operation compositions and do not require local directory ownership.
- **Portable workspace file as source of truth:** rejected for v1 because
  concurrent edits, secret references, and authority activation complicate
  normal operation.

## Consequences

- The persistence service is the single writer for all workspaces.
- Host deletion needs reference-impact UX.
- Workspace export never contains credential handles or active authority.
- The local database remains the sole source of workspace truth.

## Migration or follow-up

Implement the aggregate and lifecycle contract in
[workspace architecture](../workspaces.md) and cover global references,
revision conflicts, redacted export, and restore degradation in tests.
