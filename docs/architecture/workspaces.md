# Workspace Architecture

## Definition

A workspace is a local Relio aggregate that composes operational context. It is
not a filesystem directory, operating-system account, credential container,
database, or security principal.

A workspace owns:

- name, description, tags, and optional environment classification;
- references to global host profiles;
- tabs, panes, tool surfaces, and restoration metadata;
- workspace-scoped settings;
- workspace-owned snippets and history filters;
- references to recordings and recent operations.

Credentials remain in the OS secret store. Host profiles remain global records
that may be referenced by multiple workspaces.

## Identity and references

- Every workspace, host, session, pane, snippet, and operation has a stable
  opaque ID.
- Names and paths are mutable labels, never database identities.
- A workspace-to-host relationship is an explicit association record. It may
  carry a workspace-specific alias, role, ordering, and environment label.
- Deleting a workspace removes workspace-owned records and associations. It
  does not delete referenced global host profiles, credentials, or remote
  files.
- Deleting a host requires a separate impact preview listing workspace
  references and credential handles.
- Missing hosts and local shell executables produce unresolved references
  that the user can repair; restoration does not silently create replacements.

## Aggregate boundary

Commands that mutate a workspace go through the workspace application service.
The service:

1. loads the workspace and expected revision;
2. validates references and policy;
3. applies one domain operation;
4. writes the aggregate and an operation result transactionally;
5. emits facts after commit.

Use optimistic revision checks for stale UI requests. There is one local writer,
so distributed locking and conflict-free replicated data types are unnecessary.

## Layout model

The layout is a versioned tree:

- split nodes contain orientation, ordered children, and normalized weights;
- leaf nodes reference a surface instance;
- surfaces reference a session, file view, log view, transfer view, or
  port-forward view;
- the active leaf and navigation selection are separate state;
- every surface persists a core type identifier and bounded, versioned state.

Layout writes are debounced, but meaningful operations such as pane close or
workspace switch are flushed before completion is reported. Invalid trees
recover to a valid single-pane layout while preserving a repair diagnostic.

## Settings resolution

The effective settings context is:

```text
application safety policy
  constrains
built-in default -> user -> workspace -> host -> session
```

Precedence only applies to settings whose schema allows the scope. Safety policy
is a constraint, not another override: a lower scope cannot enable behavior
forbidden by policy. Merge behavior is declared per setting; lists and objects
do not receive an implicit deep merge.

A host setting applies after workspace settings because it describes the target
host. A session override is ephemeral unless the setting explicitly supports a
saved session profile.

## Lifecycle

Workspace states are:

```text
active -> archived -> deleted
```

- Archive hides the workspace and stops automatic restoration without deleting
  data.
- Delete shows owned records, referenced global records, active sessions, and
  retained recordings.
- Active sessions must be closed or explicitly detached according to provider
  support before deletion.
- Deletion is transactional for database records and schedules encrypted blob
  deletion. Physical secure deletion is not promised.

## Export

The workspace export is a versioned, deterministic UTF-8 JSON document
containing selected non-secret records. It is not a database copy and is not an
active workspace source.

It always excludes credential bytes, secret handles, and active credential
associations. By default it also excludes:

- recordings, terminal output, command history, and logs;
- remote paths unless the user includes them after preview;
- machine-specific runtime and temporary data.

The user previews categories and destination before writing. V1 does not import
a workspace export. Profile recovery uses the authenticated encrypted backup
flow; a user recreates a workspace or host association explicitly rather than
activating authority from a portable document.

## Restore behavior

On startup Relio restores the last active workspace and valid layout. It does
not claim to resurrect local processes. A restorable session record may offer:

- open a new local shell in the last known directory;
- reconnect to an SSH host after showing target and identity;
- leave an unavailable placeholder for user repair.

No reconnect, command replay, tunnel restart, or credential use occurs silently
after a crash.

## Scale targets

The model must remain responsive with the datasets in
[performance and capacity](performance-and-capacity.md). Lists use paging or
virtualization, relationships use indexed IDs, and search uses an explicit
index. A workspace aggregate does not load recordings, terminal bytes, or every
host detail into memory.

## Required tests

- create, rename, archive, delete, and restore;
- global host-profile reuse and deletion impact;
- revision conflict and transactional rollback;
- malformed and cyclic layout recovery;
- export redaction and proof that secret handles are absent;
- migration from every supported workspace schema;
- large-workspace list, search, and layout performance.
