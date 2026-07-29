# Window and Process Lifecycle

## Scope

This document is the implementation contract for single-instance ownership,
desktop startup, window restoration, close review, shutdown, OS-session lock,
and unclean-exit recovery. The normative state order remains in
[implementation architecture](implementation-architecture.md).

## Single-instance protocol

Relio owns one process per OS user and profile root.

- The primary process holds an exclusive operating-system file lock for its
  lifetime.
- The lock and endpoint metadata live below a user-private runtime directory.
  Unix directory/file modes are `0700`/`0600`; Windows uses the current user's
  ACL-inheriting local application-data directory.
- The primary binds an ephemeral IPv4 loopback endpoint and publishes only its
  address, protocol version, and a fresh 128-bit process-lifetime token.
- A secondary process reads metadata only while it cannot acquire the lock,
  sends one authenticated bounded intent, waits at most two seconds for an
  acknowledgement, and exits.
- Metadata is capped at 512 bytes and launch messages at 4 KiB. Unknown fields,
  versions, non-loopback addresses, bad tokens, paths, shell text, URLs, and
  unknown intent variants are rejected.
- v1 intents are `activate` and `open_workspace { workspace_id }`. The primary
  resolves the opaque workspace identifier through its own repositories.
- File locking provides crash recovery: a dead owner releases the kernel lock.
  Stale file contents never establish ownership.

Loopback is transport, not trust. Authentication derives from user-private
metadata, and the Rust action handler still validates state and authorization.

## Startup integration

The executable performs these steps before creating the webview:

1. Resolve the platform user-private runtime directory.
2. Acquire ownership or forward a bounded activation intent and exit.
3. Open lifecycle metadata and atomically write `clean_exit = false`.
4. Build the Tauri runtime using bundled assets and the compiled capability.
5. Restore only geometry that intersects a current display and respects
   `720 × 480` minimum and 16,384-pixel maximum dimensions.
6. Start the launch endpoint receiver and expose the workbench.

Profile opening, migration, and durable workspace restore remain disabled until
their owning milestones. Adding them must advance the existing
`StartupCoordinator`; it must not create a parallel bootstrap path.

Startup performs no network request other than binding the local loopback
single-instance endpoint. It performs no SSH, update, cloud, or reconnect work.

## Window geometry

Lifecycle metadata contains only schema version, clean-exit state, and window
geometry. It contains no workspace name, host, path, command, or credential
data. Writes use a user-private temporary file, `sync_all`, and atomic rename.

Persist logical user intent as physical coordinates and dimensions captured
from Tauri at shutdown. On restore:

- reject dimensions below the minimum or over the hard maximum;
- reject a rectangle that intersects no current display;
- use the bundled centered `1120 × 720` fallback after monitor removal or
  corrupt metadata;
- restore maximized state only after safe position and size;
- do not restore minimized or focused state.

## Close and shutdown

Every main-window close enters `Reviewing`. Current blockers are requested from
their authoritative services. If none exist, shutdown may immediately advance;
otherwise the trusted review surface offers feature-specific resolution and
Cancel.

The coordinator is the only legal state path:

`Running → Reviewing → Quiescing → Draining → Persisting → Exiting`

Cancel returns `Reviewing → Running`. After ten seconds, `Draining` advances to
`ForcedCleanup`; owned children get three seconds before force-stop. New work is
rejected from `Quiescing` onward. `clean_exit = true` is persisted only from the
final exit path after safe metadata has been captured.

Future blocker owners register typed shutdown participants. They may not add
independent window-close handlers.

## Session lock and webview loss

Platform adapters translate only authenticated OS session notifications into
`Locked` and `Unlocked`; ordinary focus loss is not a session lock.

On `Locked`, the core:

- increments the authority epoch;
- consumes every pending confirmation;
- revokes secret leases;
- pauses or cancels credential-dependent pending work according to provider
  policy;
- notifies the frontend to obscure sensitive views.

`Unlocked` restores visibility but never recreates leases or approval
authority. Webview loss increments the authority epoch and consumes pending
confirmations, but does not claim that the OS session is locked.

Platform adapters are exercised in Tier 1 native CI because Linux
logind/desktop-bus, Windows session-change, and macOS workspace notifications
are unavailable in the platform-neutral test runtime.

## Recovery

At process start, Relio reads the previous marker and immediately writes an
unclean marker. If the previous marker is unclean, recovery runs before
workbench restoration and must not claim that incomplete operations succeeded.
Corrupt or oversized metadata falls back safely and produces a redacted
diagnostic.

The marker becomes clean only at structured exit. Process kill, power loss,
panic, or OS termination therefore leaves evidence for the next startup.

## Required verification

- ownership race, stale-lock recovery, and user-only permissions;
- valid forwarding plus spoofed, oversized, malformed, unknown, and
  non-loopback messages;
- every startup and shutdown transition, failure edge, cancel, and deadline;
- monitor removal, corrupt geometry, minimum size, and maximized restore;
- persisted forced-termination marker and atomic clean-exit replacement;
- lock/unlock and webview-loss authority epoch behavior;
- native launch, focus, close, lock, forced-kill, and restart tests on Ubuntu
  24.04, Windows 11, and macOS Sonoma.
