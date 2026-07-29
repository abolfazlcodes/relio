# Secret Management

## Scope

A secret is data whose disclosure can authorize an action or reveal protected
content: SSH passwords, private keys, key passphrases, database keys,
application/update signing keys, and recovery keys.
Hostnames and usernames are sensitive metadata but are not secret authority.

Credential storage is described in [credential security](credentials.md).
This document defines how application code is allowed to acquire and use secret
material.

## Core rule

Secret material stays out of the webview, ordinary settings, workspace records,
frontend IPC, command lines, URLs, logs, product analytics, crash reports,
clipboard, and long-lived application events.

An opaque secret handle:

- is random and non-semantic;
- identifies purpose and owner in the core;
- grants no authority by itself;
- is rejected outside its declared profile, credential type, and target scope;
- is redacted from normal export because it can reveal relationships even when
  it cannot reveal the secret.

## Secret service boundary

Only the Rust secret service may ask a platform adapter for secret bytes. A
caller provides:

- secret handle;
- operation ID;
- purpose such as `ssh.authenticate` or `database.open`;
- target identity;
- expected secret type;
- maximum lease lifetime.

The service revalidates policy and returns a non-cloneable, short-lived lease to
an approved core adapter. Leases cannot be serialized or sent over frontend IPC
or to unrelated processes.

## Lifecycle

1. Generate or receive the secret at a trusted core boundary.
2. Store it through the OS secret facility when persistence is approved.
3. Store only its opaque handle and classification in the encrypted database.
4. Resolve it for one authorized operation.
5. Pass it through a protected API, agent, standard input, or authenticated
   helper channel; never an argument or ordinary environment variable.
6. Close the lease and zero owned buffers where the platform and library make
   that reliable.
7. Rotate references transactionally and revoke obsolete keychain items after
   successful validation.
8. Delete on explicit user request and report partial failure.

Memory clearing reduces accidental retention but does not defeat a compromised
OS, debugger, swap, crash dump, or language/runtime copy. Relio must not claim
otherwise.

## Process handoff

When a native API or agent can perform the operation without revealing secret
bytes to Relio, prefer it.

When OpenSSH requires an interactive answer, use a Relio-owned askpass helper:

- the environment may contain only the helper path, operation ID, and an
  unguessable one-time channel identifier—not the answer;
- the helper connects to a user-only local IPC endpoint;
- the core verifies helper process identity where the platform permits;
- one prompt consumes one scoped response and the channel then closes;
- prompt text is bounded, classified, and never treated as trusted markup;
- cancellation and timeout produce an authentication cancellation.

Password authentication remains unavailable until this helper path passes the
security test matrix.

## User input and clipboard

- Use native secure-input behavior where available.
- Do not provide a default “show password” control for secrets that the user
  does not need to verify; where provided, require a deliberate press-and-hold
  or equivalent clear state.
- Disable copying generated secrets by default. If the user explicitly copies,
  warn that clipboard managers may retain the value and attempt timed clearing
  only when the clipboard still contains the same value.
- Never paste a secret into a terminal or remote form automatically.
- Do not save secret form values during crash recovery or navigation.

## Rotation and revocation

- Database and content keys follow [encryption strategy](encryption.md).
- Credential rotation creates and verifies the replacement before switching the
  owning record.
- Revocation removes future authority immediately even when OS keychain
  deletion later fails.
- A rotated or deleted handle produces a stable “credential unavailable” error,
  not fallback to a different credential.
- Release and platform signing keys never enter the Relio desktop application;
  their lifecycle belongs to controlled release infrastructure.

## Diagnostics and tests

Required tests cover:

- secret absence from all serialized DTOs, events, exports, support bundles, and
  logs;
- keychain denial, lock, duplicate, corruption, and deletion failure;
- lease scope, expiry, replay, cancellation, and concurrent use;
- askpass spoofing, oversized prompts, stale channels, and child crashes;
- clipboard opt-in and safe clearing behavior;
- rotation rollback and revoked-handle denial.
