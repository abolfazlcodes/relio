# IPC and Process Model

## Purpose

This document defines authority, process isolation, contract ownership,
streaming, cancellation, and shutdown. It is security-sensitive because a
webview compromise must not become unrestricted OS access.

The concrete command inventory, envelopes, confirmation challenge, stream
protocol, reconciliation rules, and lifecycle contracts are normative in
[implementation architecture](implementation-architecture.md). This document
defines their security rationale.

## Process topology

### Trusted desktop process

The Tauri/Rust process owns application policy, persistence, secrets,
transports, process supervision, and updates. Rust code is
inside the trusted computing base and must still validate all less-trusted
input.

### Less-trusted webview

The React workbench renders bundled application assets and expresses user
intent. It is treated as potentially compromised by a rendering bug, injection,
malicious remote content, or unsafe dependency.

The webview:

- loads no remote application code;
- has no generic shell, filesystem, network, database, secret, or updater
  install command;
- receives only display data and opaque handles required for the active view;
- treats theme data, remote names/output, and imported content as untrusted
  display data.

### Supervised child processes

The core may supervise:

- local shells and PTYs;
- OpenSSH and narrowly scoped helper processes;

Every child has an owner, start deadline, health state, cancellation path,
graceful-stop deadline, force-stop fallback, and final reaping responsibility.
No orphan is considered acceptable normal behavior.

## Tauri exposure policy

- List each window or webview explicitly in Tauri capability configuration.
- Enable capabilities explicitly in application configuration; do not rely on
  auto-discovery of every capability file.
- Register only application commands intended for that window.
- A Tauri capability permits an IPC route; application services still enforce
  scope, state, and user policy.
- Remote API access is disabled. No remote origin receives Tauri commands.
- The content security policy defaults to `self`, denies remote scripts and
  frames, and allows only the minimum image/font/style sources required by
  bundled assets.
- The asset protocol is disabled unless a feature has a narrow path scope and a
  security review. User and remote files are not exposed as arbitrary webview
  URLs.

## Command contract

Every command has:

- a stable command name;
- a generated request and response type;
- validation limits for every string, collection, path, and payload;
- documented side effects and required application policy;
- an operation ID if work can outlive the request;
- cancellation and timeout semantics;
- typed machine-readable errors and a separate user-safe message.

The canonical DTOs are Rust types plus generated schema/TypeScript bindings.
The build fails when generated bindings are stale. Dynamic untyped value maps
are prohibited at privileged boundaries and allowed only inside explicitly
versioned user-data fields.

Commands express intent, for example `workspace.create` or
`session.request_input`; they do not expose implementation primitives such as
`execute_sql`, `read_path`, or `spawn`.

## Authorization and confirmation

Authorization has three distinct layers:

1. **Build-time exposure:** whether this webview can invoke the command at all.
2. **Application policy:** whether the current target, state, and principal may
   perform the action.
3. **User decision:** whether a material operation requires a final, trusted
   confirmation.

A confirmation is produced by core-owned UI state. Theme settings and untrusted
content cannot create, approve, obscure, or imitate the trusted confirmation
surface. Replaying a previous request ID or operation ID does not repeat a
privileged action.

## Events

Events report facts such as `session.state_changed` or
`operation.completed`. They include:

- stable aggregate and operation IDs;
- a monotonic sequence within the owning stream when ordering matters;
- UTC timestamp for diagnostics, not ordering;
- schema version where the event crosses a public contract;
- a bounded payload.

Events do not carry credentials, raw private paths unrelated to the active
view, or unbounded logs. Subscribers tolerate additive fields and dispose their
subscriptions with the owning view.

## Terminal and bulk streams

Terminal bytes, file transfers, recordings, and large logs do not use the
general event bus.

- Streams are binary where practical and have bounded chunks.
- The producer sends only while it has receiver credit or an acknowledged
  window.
- Per-stream queues have fixed byte and age limits.
- Ordering is preserved within a stream; cross-stream ordering is not implied.
- Cancellation closes the stream and transitions the owning operation exactly
  once.
- A detached terminal DOM keeps its frontend terminal model alive. If the
  entire frontend disconnects, the backend retains only a bounded replay
  window.
- When replay capacity is exhausted, the runtime applies transport
  backpressure. If a provider cannot pause safely, it marks an explicit output
  gap or ends the session according to the provider policy; it never grows
  memory without bound.

Recording is an independent sink. Enabling recording must not be necessary for
correct live rendering.

## Error model

Errors contain:

- stable error code;
- subsystem and operation ID;
- retryability and whether user action is required;
- safe user message;
- diagnostic cause chain retained in the core.

Raw OS, SSH, and database messages are normalized before crossing to
the webview. Diagnostics apply structured redaction and remain best effort.

## Concurrency and shutdown

- Use structured concurrency: every asynchronous task belongs to an operation,
  session, transfer, or application lifetime.
- Use bounded channels. An unbounded channel requires an ADR.
- Cancellation is cooperative first and escalates after a documented deadline.
- Database writes are serialized by the persistence service.
- Shutdown stops accepting new operations, persists safe metadata, cancels
  children, flushes encrypted recording segments, checkpoints the database, and
  then exits.
- Forced OS termination may interrupt cleanup; startup recovery must detect
  incomplete operations and temporary files.

## Required tests

- command allow/deny coverage for each webview;
- malformed, oversized, replayed, and out-of-state requests;
- event ordering and unknown-field compatibility;
- stream backpressure, reconnect, cancellation, and overflow;
- child crash, timeout, orphan prevention, and shutdown escalation;
- CSP and remote-origin denial;
- secret and path absence from frontend payloads and diagnostics.

## Framework references

- [Tauri capabilities](https://v2.tauri.app/security/capabilities/)
- [Tauri content security policy](https://v2.tauri.app/security/csp/)
- [Tauri asset protocol scope](https://v2.tauri.app/security/asset-protocol/)
