# Local Database and Data Security

## Data locations and ownership

Relio uses platform-appropriate application data directories with restrictive permissions. The application must not store operational data in the repository, arbitrary temporary directories, or world-readable locations.

## Storage classification

| Data | Default handling | Additional controls |
| --- | --- | --- |
| Workspace and host metadata | Encrypted SQLite with migrations | Native ACLs, export redaction, integrity checks |
| Settings | Encrypted SQLite through the settings service | Scoped permissions, schema validation, no secret values |
| Credential material | OS keychain only | Re-authentication and opaque handles |
| Session/command history | Disabled until the user enables retention; encrypted metadata thereafter | Sensitive-output warning, retention, delete action |
| Recordings and logs | Opt-in encrypted segments with encrypted SQLite metadata | Retention, preview before export |
| Unsaved remote-edit buffer | Memory only | 10 MiB hard limit, plain-text rendering, clear on close, no crash copy |
| Crash and diagnostic data | Minimal, opt-in where it leaves device | Preview, redaction, no raw session data by default |

## SQLite protection

Relio uses a SQLCipher-compatible SQLite build. SQLite is still an organization
and transaction layer; encryption is a separate configured property that must
be tested. The baseline is:

- store the database in the OS application-data directory;
- apply restrictive native ACLs;
- retrieve a random profile key from the OS secret store;
- verify database, journal/WAL, temporary, backup, and migration files are
  encrypted;
- use transactions, journaling, migrations, and recovery backups;
- keep credentials out of the database;
- keep large recordings in separately envelope-encrypted immutable segments;
- never invent cryptography or derive a database key from a predictable device value.

There is no plaintext persistent fallback. Full-disk encryption remains
recommended defense in depth but is not assumed or required to make the Relio
format confidential.

See [persistence architecture](../architecture/persistence.md) and
[encryption strategy](encryption.md).

## Configuration and exports

Configuration and workspace exports are versioned, previewable, and redacted.
They are not assumed encrypted unless the user deliberately chooses the
encrypted profile-recovery format. They show data categories before writing to
a user-selected path and always exclude credential bytes and handles. V1 does
not ingest workspace or theme exports.

## History, logs, and redaction

Terminal output is sensitive by default because users may paste secrets or
receive tokens from commands. Relio recording and derived command history are
disabled by default, have explicit per-workspace/session controls, retention
settings, and deletion. Redaction uses known secret values and conservative
pattern detection, but the UI must state that redaction is best effort.

Support bundles exclude session content unless the user explicitly selects it after a preview.

## Secure deletion

Deleting a database row or file does not guarantee physical deletion from SSDs, snapshots, backups, filesystem journals, or disk images. Prefer short retention, encrypted sensitive data, and cryptographic deletion of the encryption key where feasible. Document this limitation rather than promising a wipe the application cannot verify.

## Backups and recovery

Backups must be opt-in and clearly classified. Internal migration/recovery
backups remain encrypted and bounded. A portable backup needs a separately
usable recovery key and cannot include plaintext credentials. Restore requires
authentication, integrity and schema validation and never overwrites active data
without confirmation.
