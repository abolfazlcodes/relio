# SSH Architecture

## Goals

Provide reliable, understandable SSH workflows while preserving common
OpenSSH behavior and secure host identity. SSH is a transport, not the
definition of a workspace.

## Provider contract

The application contract describes capabilities, not an assumption that every
provider implements every feature:

- capability discovery and diagnostic version;
- connect, authenticate, cancel, and disconnect;
- interactive channel with PTY dimensions;
- non-interactive command execution;
- SFTP file operations and SCP transfer when supported;
- local, remote, and dynamic forwarding when supported;
- host-key status and verification evidence;
- jump/proxy chain metadata;
- timeout, reconnect policy, typed errors, and process ownership.

Each provider reports supported features before the UI offers them. Unsupported
behavior is unavailable with a diagnosis; the UI does not emulate it by
building a shell command.

## First provider

Use a supported system OpenSSH client as the first interactive provider where
available. Relio supervises the executable directly with structured arguments
and a filtered environment. It owns cancellation, helper channels, output
classification, process-tree cleanup, and capability diagnosis.

This decision favors interoperability but accepts that OpenSSH's human-readable
stderr and prompts are not a stable machine protocol. Relio supports and tests a
declared OpenSSH version range per platform. Unknown versions are diagnosed and
may run only through a clearly labeled compatibility mode after testing; Relio
does not parse arbitrary output optimistically.

On systems without a supported executable, SSH features remain unavailable
while the local terminal continues to work. A native provider may be introduced
behind the same contract by a later ADR.

## Configuration strategy

Do not hand an arbitrary imported config directly to OpenSSH. OpenSSH
configuration may include executable behavior such as proxy or match commands.

The v1 flow is:

1. read only user-selected files and the standard user configuration after
   explicit opt-in;
2. parse a documented safe subset without shell evaluation;
3. resolve includes within approved paths with cycle, depth, count, and size
   limits;
4. show effective connection values and unsupported directives;
5. reject executable and unknown directives;
6. render a minimal generated config in a protected runtime directory;
7. invoke OpenSSH with the generated config and explicit safety overrides.

`LocalCommand`, `PermitLocalCommand`, `Match exec`, arbitrary environment
hooks, unknown directives, and `ProxyCommand` are unsupported in v1. Use
`ProxyJump` for supported jump-host workflows.

Relio never edits the user's source SSH config automatically.

## Host model

A global host record contains:

- alias and display name;
- address, port, username;
- groups, tags, and environment classification;
- jump-host references;
- selected provider and terminal profile;
- host-key policy/reference;
- credential handles.

It contains no plaintext passwords or private keys. Workspaces reference host
IDs and may add a workspace-specific role or alias. Exports redact addresses
and handles by default and never contain credential bytes.

## Authentication helpers

Authentication preference:

1. hardware-backed or existing agent identity;
2. external user-selected private-key file;
3. provider-supported protected credential through the Relio askpass helper;
4. password authentication only after the helper path is implemented and
   security-tested.

No password or passphrase appears in an argument or ordinary environment
variable. The askpass helper receives only a one-time local channel identifier,
connects to a user-only core endpoint, and consumes one purpose-bound response.
See [secret management](../security/secrets.md).

Agent forwarding is off by default and enabled only after the consent defined
in [SSH security](../security/ssh.md).

## Host-key flow

Relio uses a controlled known-hosts store for Relio-managed profiles while
allowing a read-only view of approved user known-hosts sources.

Connection flow:

1. determine canonical host, port, jump chain, and known-host aliases;
2. invoke the supported provider with strict verification and the Relio askpass
   bridge where a provider prompt is required;
3. display algorithm, SHA-256 fingerprint, source, host/port, and jump context
   in trusted core-owned UI;
4. write a first-use decision only after explicit approval;
5. fail closed on changed, revoked, malformed, or ambiguous keys;
6. record verification history without secret session content.

Do not use unauthenticated `ssh-keyscan` as proof of identity. It may collect a
candidate key for out-of-band comparison, but trust still comes from the user's
verification or an already trusted source.

## Interactive sessions

- Allocate a PTY only for interactive sessions.
- Treat remote bytes as untrusted terminal data.
- Separate transport lifecycle from pane/layout lifecycle.
- A reconnect creates a new transport connection and never silently replays
  commands or restarts tunnels.
- Keepalive settings are bounded and visible; failure transitions to a typed
  disconnected state.
- Closing a pane follows the configured session policy and shows when a live
  remote process may be terminated.

## SFTP, SCP, and remote editing

The OpenSSH interactive process does not imply a reusable file-transfer
channel. V1 starts a separate supported OpenSSH process for the SFTP subsystem
and speaks the binary SFTP protocol over protected standard streams through a
maintained, bounded Rust protocol implementation. It does not parse localized
human-readable `sftp` command output.

This reuses OpenSSH configuration, host-key verification, agent behavior, and
authentication while avoiding a second SSH cryptographic stack. The protocol
implementation limits packet length, outstanding requests, directory page
size, path length, transfer buffers, and timeouts. It preserves remote path
bytes in the core and converts them to display text only at the UI boundary.
The UI reports that file operations use a separate authenticated connection and
does not claim connection sharing.

For SCP workflows, use a supervised `scp` invocation only when capability
diagnosis proves that the supported executable uses SFTP semantics. The
operation accepts one literal source and destination, uses structured process
arguments, and rejects ambiguous option-like or wildcard paths. The legacy SCP
protocol is not supported in v1 because its remote-shell path interpretation
and wildcard behavior create avoidable ambiguity and attack surface. Relio
reports the active transfer semantics rather than inferring them from the
executable name.

The common transfer contract exposes source/destination, progress,
cancellation, overwrite policy, post-transfer verification where available,
and a terminal result. If the external command cannot provide trustworthy
fine-grained progress, the UI reports indeterminate progress rather than
parsing unstable output.

Remote editing:

1. stat the target and refuse directories, devices, and unsupported file types;
2. download at most 10 MiB into the bounded built-in text editor with a version
   identity; reject NUL-containing or invalid UTF-8 content for editing;
3. treat content as plain text, preserve detected line endings, and never render
   it as HTML or active markup;
4. retain the unsaved buffer in memory only; v1 creates no local plaintext
   draft or crash-recovery copy and does not launch an external editor;
5. re-stat/re-identify the remote target before save;
6. show conflict, symlink, permissions, ownership, and overwrite behavior;
7. upload to a temporary remote sibling where supported;
8. atomically rename when remote semantics permit;
9. report when atomic replacement or metadata preservation is unavailable and
   require separate confirmation before any non-atomic direct overwrite.

Larger, binary, or unsupported-encoding files can be downloaded but not edited
inside Relio v1. The 10 MiB editor limit is a hard safety limit, not an ordinary
setting. Buffer clearing on close is best effort and cannot defeat a
compromised webview, process memory inspection, or operating-system paging.

## Port forwarding

A forwarding operation records direction, local bind, remote destination,
transport host, jump chain, state, owner workspace, and lifecycle timestamps.

- Loopback is the default bind.
- Broad binds require trusted confirmation and may require re-authentication.
- Control sockets live in a private runtime directory with randomized,
  length-bounded paths and restrictive ACLs.
- Reconnect never duplicates a listener. The supervisor reconciles one desired
  operation with at most one active listener.
- Stop verifies that the owned listener/process ended; it does not kill an
  unrelated process by port number.

## Failure taxonomy

Errors distinguish:

- executable absent or unsupported;
- configuration invalid or executable directive blocked;
- keychain/agent/helper unavailable;
- authentication rejected;
- unknown, changed, or revoked host key;
- DNS, route, timeout, proxy, or jump-hop failure;
- remote command/channel failure;
- local listener conflict or privilege denial;
- cancellation and process cleanup failure.

Raw stderr remains diagnostic input, not the user-facing contract.

## Required tests

- supported OpenSSH versions on every Tier 1 platform;
- safe config subset, include cycles/limits, and executable directive denial;
- structured argument handling for hostile aliases, paths, usernames, and
  options;
- askpass spoofing, cancellation, timeout, and secret absence;
- unknown/changed/revoked/hashed host-key cases and jump chains;
- PTY resize, disconnect, reconnect, and process-tree cleanup;
- malformed/oversized SFTP packets, request-ID confusion, directory paging,
  cancellation, and server timeout;
- SFTP/SCP partial transfer, hostile/non-text path, symlink, conflict,
  overwrite, and atomicity differences;
- legacy SCP refusal on every host and supported platform;
- loopback/broad forwarding, duplicate prevention, and orphan cleanup.
