# Architecture Decision Record Index

ADRs preserve why a durable decision was made. They are historical records:
accepted ADRs are superseded, not edited into a different decision.

## Index

| ADR | Decision | Status | Location |
| --- | --- | --- | --- |
| 001 | Tauri 2 desktop shell | Accepted | [Legacy decision collection](../technology-decisions.md#adr-001-tauri-2-for-the-desktop-shell) |
| 002 | React and TypeScript workbench | Accepted | [Legacy decision collection](../technology-decisions.md#adr-002-react-and-typescript-for-the-workbench) |
| 003 | xterm.js initial renderer | Accepted | [Legacy decision collection](../technology-decisions.md#adr-003-xtermjs-as-the-first-terminal-renderer) |
| 004 | OpenSSH-first transport adapter | Accepted direction | [Legacy decision collection](../technology-decisions.md#adr-004-adapter-based-transport-with-openssh-compatibility-first) |
| 005 | Encrypted SQLite persistence | Accepted | [Legacy decision collection](../technology-decisions.md#adr-005-sqlite-for-local-metadata) |
| 006 | No dynamic application-code loading in v1 | Accepted | [Legacy decision collection](../technology-decisions.md#adr-006-no-dynamic-application-code-loading-in-v1) |
| 007 | Encryption at rest | Accepted | [ADR-007](007-encryption-at-rest.md) |
| 008 | Workspace persistence | Accepted | [ADR-008](008-workspace-persistence.md) |
| 009 | Update trust | Accepted | [ADR-009](009-update-trust.md) |

ADRs 001–006 predate the one-record-per-file convention and remain embedded in
the technology decision collection to avoid duplicating or rewriting accepted
history. ADR-010 and later use one zero-padded file per decision.

## Creating an ADR

Copy [the template](000-template.md), use the next number, and add it to this
index. The record includes status, context, decision, alternatives,
consequences, security/privacy impact, compatibility/migration, reversal cost,
owner, and review triggers. A superseding ADR links both directions.
