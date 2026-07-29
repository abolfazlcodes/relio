# ADR-010: Portable PTY as the Native PTY Adapter

- **Status:** Accepted
- **Date:** 2026-07-29
- **Owner:** Runtime maintainers (`@owner` until replaced)

## Context

Relio needs one maintainable interface over Windows ConPTY and POSIX PTYs.
Correct PTY allocation, controlling-terminal behavior, handle duplication,
resize, process waiting, and platform error handling require substantial
platform-specific unsafe code. Relio forbids project-owned unsafe code and
needs Tier 1 parity before terminal rendering begins.

The standard library has process pipes but no PTY or ConPTY API. Direct
`windows`, `nix`, and `libc` implementations would place a large,
security-sensitive compatibility surface inside Relio. Shelling out to
`script`, terminal emulators, or command interpreters is not portable and would
weaken structured argument and process ownership guarantees.

## Decision

Use exactly `portable-pty 0.9.0` behind a Relio-owned `PtyAdapter` port.
`portable-pty` is maintained as part of WezTerm and selects ConPTY on Windows
and a native POSIX PTY implementation on Unix.

Relio does not expose dependency types outside the infrastructure adapter. The
application runtime owns:

- shell-profile validation and structured argv construction;
- session IDs, state, capacity, cancellation, and shutdown deadlines;
- bounded input/output queues, byte credit, replay, and gap policy;
- safe public error normalization and diagnostics;
- process-tree conformance tests and any stronger platform supervision needed
  beyond the dependency's child handle.

The dependency is pinned and default features are accepted for native PTY and
serial implementation compatibility; Relio does not expose serial sessions in
v1. No optional serialization or remote-SSH feature is enabled.

## Dependency review

- **Capability:** PTY/ConPTY allocation, resize, reader/writer handles, child
  spawn/wait/kill.
- **Why not standard library:** it has no PTY APIs.
- **Maintenance:** part of the actively maintained WezTerm project; 0.9.0 was
  released in 2025.
- **Licenses:** dependency and transitive licenses must remain accepted by the
  repository license gate and generated SBOM.
- **Native/unsafe code:** platform FFI and unsafe code exist transitively,
  isolated behind the adapter; no lifecycle script or network activity occurs
  at runtime.
- **Impact:** adds PTY platform dependencies and modest build/binary cost; no
  startup work occurs until a user requests a local session.
- **Trust boundary:** receives validated executable paths, argv, environment,
  cwd, and dimensions; returns only owned handles, bytes, and normalized exit
  facts.
- **Test seam:** the application depends on a project-owned fakeable port.
- **Supporting containment dependencies:** pinned `nix 0.28.0` exposes the safe POSIX process-group signal API and pinned `win32job 2.0.3` exposes a safe Windows kill-on-close Job Object wrapper; both are target-specific and remain behind the same adapter.
- **Replacement:** implement the same port with another adapter, pass the PTY
  conformance suite, then remove the crate and transitive lockfile entries.

## Alternatives

- **Direct platform APIs:** rejected because lifetime FFI and process-tree
  maintenance cost is high and conflicts with the no-project-unsafe policy.
- **Plain child pipes:** rejected because interactive programs require terminal
  semantics, resize, job control, and ConPTY.
- **External terminal helper:** rejected because deployment, protocol,
  supervision, signing, and attack surface would exceed an in-process adapter.

## Consequences

Relio gains one testable cross-platform adapter and avoids leaking WezTerm
types into domain/application code. It still owns process-tree and orphan
conformance; a successful child `kill` is not by itself evidence that all
descendants were reaped.

Upgrades require focused PTY compatibility, process cleanup, dependency,
license, and security review. If the crate becomes unmaintained or cannot meet
Tier 1 cleanup guarantees, this ADR enters replacement review.

## Security and privacy

Programs and arguments are never concatenated into a shell command. Environment
and cwd values are bounded and validated. Terminal output remains hostile bytes
and does not authorize actions. The adapter runs only at user privilege and
must terminate only process trees Relio created.

## Compatibility and migration

No durable user data stores dependency types. Runtime sessions do not survive
application restart, so adapter replacement needs no data migration.

## Reversal cost

Medium. The project-owned port and fake conformance suite are the replacement
boundary; native platform behavior still requires a full Tier 1 qualification.

## Review triggers

- a reachable security advisory or maintenance inactivity;
- changes to ConPTY/POSIX process-tree behavior;
- new native build scripts, licenses, default features, or unsafe surface;
- inability to pass orphan, resize, pressure, or shutdown tests;
- a materially smaller and better-supported standard or framework API.
