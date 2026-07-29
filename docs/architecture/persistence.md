# Persistence Architecture

## Scope

Persistence stores local metadata and indexes. It does not own live processes,
terminal renderer state, credentials, or remote truth.

## Storage layout

One local Relio profile contains:

```text
profile/
├── relio.db                 # encrypted SQLite database
├── blobs/                   # encrypted immutable recording/log segments
├── cache/                   # disposable, non-secret, integrity-checked data
├── recovery/                # bounded encrypted migration backups
└── logs/                    # minimal diagnostics with retention
```

Exact roots use platform application-data and cache conventions. Data is never
stored in the source repository or current working directory.

## Database decision

Use a SQLCipher-compatible SQLite build for the application database. Supply a
random 256-bit database key from the OS secret store through the database API;
never use a user password, device identifier, URI, command-line argument, or
environment variable as database key material.

The selected distribution must pass:

- Windows, macOS, and Linux build/signing tests;
- database, WAL, journal, temporary-file, and backup encryption tests;
- license and attribution review;
- vulnerability and maintenance review;
- measured startup and query budgets.

If these gates cannot be met, implementation stops for a replacement ADR. It
must not silently fall back to plaintext for real profiles.

## Connection and concurrency model

- The desktop process is the only database writer.
- A profile lock prevents a second writer. A second launch forwards its intent
  to the first process or opens a clearly labeled recovery/read-only flow.
- Begin with one dedicated database worker connection. This is simpler than a
  pool and matches SQLite's single-writer behavior.
- Enable a separate read connection only after profiling demonstrates need and
  WAL/read consistency tests exist.
- UI and session tasks never execute SQL directly; they call repository
  operations through the persistence service.
- All queries are parameterized. Runtime-loadable SQLite modules and arbitrary
  SQL execution outside repository code are disabled.

## Schema ownership

Tables are grouped by domain ownership:

- workspace and layout;
- host and workspace-host association;
- settings and policy metadata;
- session and operation metadata;
- snippets and search index;
- credential references;
- recording/blob index and retention.

Foreign keys are enabled. Cascades are used only when aggregate ownership makes
deletion unambiguous. Global records referenced by multiple local aggregates
require explicit service-level deletion.
Timestamps are UTC; ordering uses database revisions or sequences rather than
wall-clock time.

## Migrations

Each schema change has a monotonically increasing migration ID and an upgrade
test from the oldest supported schema.

Migration procedure:

1. acquire the exclusive profile lock;
2. verify key access and database integrity;
3. create a bounded encrypted recovery copy;
4. run the migration transaction where SQLite permits;
5. validate schema version, foreign keys, and critical invariants;
6. atomically mark the new schema active;
7. retain the previous backup according to rollback policy.

Migrations are forward-only. The previous application may not open a newer
schema. Rollback therefore means restoring the pre-upgrade encrypted backup
after explicit confirmation that post-upgrade changes will be lost.

A failed migration never opens the database in a partially writable state.
Relio offers retry, diagnostic export without content, or recovery from the
known-good copy.

## Large and sensitive content

Terminal recordings and imported logs do not become unbounded SQLite blobs.

- Write immutable encrypted segments to the blob store.
- Use a temporary file in the same protected directory and atomically rename
  after authentication/integrity metadata is durable.
- Store content ID, owner, size, encryption version, checksum/authentication
  metadata, retention, and state in SQLite.
- A background janitor removes abandoned temporary segments and database-orphan
  blobs after a grace period.
- Search indexes contain only data the retention policy permits and are
  encrypted with the database.

See [encryption strategy](../security/encryption.md).

## Integrity, backup, and recovery

- Run a quick integrity check after unclean shutdown and a full check before
  migration or backup.
- Set conservative busy timeouts and surface lock failures; do not retry
  forever.
- Bound recovery backups by count and age.
- Backups remain encrypted under a separately wrapped backup key so they can be
  rotated and restored deliberately.
- A normal export is a schema-validated transfer format, not a raw database.
- Never open an arbitrary user-supplied SQLite file as the active database.

## Keychain-unavailable behavior

If the profile key cannot be retrieved because the keychain is locked,
unavailable, or denied:

- do not create a new key or plaintext database;
- keep the profile closed;
- explain the platform error without revealing secret metadata;
- allow a temporary no-persistence local terminal only if it uses a separate
  empty runtime and the user explicitly chooses it;
- retry only after a user action or keychain state change.

If the key is permanently lost, encrypted data is unrecoverable unless the user
has an independently usable encrypted backup. This is a deliberate
confidentiality tradeoff and must be clear during backup setup.

## Required tests

- encrypted-at-rest inspection of database, WAL, journal, temporary, blob, and
  recovery files;
- wrong, missing, locked, rotated, and revoked key behavior;
- concurrent-launch writer exclusion;
- transaction rollback, disk-full, interrupted write, and corruption recovery;
- migration success and failure from every supported schema;
- orphan blob and temporary-file cleanup;
- retention deletion and export redaction;
- query and startup performance on the reference datasets.
