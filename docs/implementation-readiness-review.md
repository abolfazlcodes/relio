# Implementation Readiness Review

## Verdict

Relio v1 is implementable with the selected Tauri, React, and Rust architecture.
No finalized UX requirement is technically impossible. Implementation must use
the canonical contracts in
[implementation architecture](architecture/implementation-architecture.md).
That document resolves the ownership, IPC, lifecycle, error, logging, testing,
and build ambiguities found during the repository-wide review.

This review covers every repository document present on 2026-07-29. It does not
approve production code or replace an ADR when implementation changes a
recorded decision.

## Resolved findings

| Finding | Resolution |
| --- | --- |
| Principles existed without a complete component graph | Added a normative graph and forbidden dependencies |
| IPC lacked a concrete inventory and common envelopes | Added versioned query, command, decision, event, stream, and error contracts |
| “Core-owned confirmation UI” could imply Rust renders UI | Rust owns and validates the challenge; reserved bundled React UI only renders it |
| Frontend state ownership was too broad | Defined stores, authority, revision reconciliation, and terminal-stream isolation |
| Rust service/task ownership was unspecified | Defined modules, ports, supervisors, cancellation hierarchy, and single-writer rules |
| Startup and shutdown order was incomplete | Added exact state machines, readiness criteria, deadlines, and recovery |
| Domain lifecycles were split across documents | Added normative connection, session, theme, workspace, settings, transfer, and remote-edit states |
| “Test connection” could mutate trust or create a session | It authenticates and probes, stores only an expiring result, and never silently changes trust |
| Stream transport was abstract | Added a credit-based binary contract behind a Tauri channel adapter |
| Transfer conflict, partial-file, and resume behavior was incomplete | Added preflight identity, temporary destination, explicit conflict policy, verification, promotion, and cleanup |
| Remote editing lacked version identity/save semantics | Added version tokens, memory-only buffers, conflict states, and transactional save |
| Runtime hard limits were incomplete | Added versioned defaults and required every queue/parser/cache to declare a limit |
| Errors and logs lacked stable schemas | Added error taxonomy, propagation, structured logging, redaction, retention, and correlation |
| Testing lacked harness and CI topology | Added fixture ownership, platform lanes, contract checks, fault injection, and release evidence |
| Build architecture was absent | Added workspace topology, pinned inputs, generation order, target matrix, signing separation, and promotion |
| Single-instance forwarding was unspecified | Added a user-only authenticated endpoint and a narrow launch-intent schema |
| Restore could be mistaken for process resurrection | Only layout and descriptors persist; process and scrollback continuity are never claimed |

## Conflict precedence

If documents disagree, use this order:

1. security invariants and threat model;
2. v1 scope in `future-ideas.md` and the product feature map;
3. this review and the implementation architecture;
4. focused architecture documents;
5. UX documents;
6. roadmap sequencing and examples.

Examples are non-normative when they conflict with a normative table. A
conflict discovered during implementation requires a documentation change or
ADR; engineers must not silently pick an interpretation.

## Deliberate v1 non-features

- No plugin runtime, marketplace, AI assistant, cloud sync, hosted service, or
  remotely loaded application code.
- No exact resurrection of local or SSH processes after restart.
- No general editor, generic shell/process/filesystem/SQL/network IPC, or
  arbitrary updater command.
- No plaintext profile fallback and no private-key import into Relio storage.
- No legacy SCP protocol. An `scp` executable is usable only when Relio proves
  it will use SFTP semantics.
- No workspace or theme import. Export formats remain versioned for a later,
  separately designed import feature.

## External release prerequisites

These do not block architecture work, but block the stated release gate:

- choose and publish the license before accepting implementation dependencies
  or publishing source;
- name the private security and incident owner before public binaries;
- name application, update, platform-signing, and key-recovery owners before a
  signed preview;
- publish Tier 1 reference machines before performance budgets become release
  gates;
- pin exact toolchains and security-critical library versions in the first
  scaffold ADR and lockfiles.

These are governance inputs, not decisions an implementing engineer may make
inside code.

## Definition of ready

A feature work item must name:

- owning frontend feature and Rust application service;
- commands, queries, events, and streams used;
- persisted records and revision behavior;
- lifecycle states, terminal outcomes, cancellation, timeout, and recovery;
- capability, confirmation, and data classification;
- platform support and degraded behavior;
- resource limits and performance budget;
- unit, contract, integration, E2E, security, and observability evidence.
