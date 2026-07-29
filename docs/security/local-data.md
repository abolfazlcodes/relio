# Local Data Security

## Data locations and ownership

Relio uses platform-appropriate application data directories with restrictive permissions. The application must not store operational data in the repository, arbitrary temporary directories, or world-readable locations.

## Storage classification

| Data | Default handling | Additional controls |
| --- | --- | --- |
| Workspace and host metadata | SQLite with migrations | ACLs, export redaction, integrity checks |
| Settings | SQLite or versioned config through the settings service | Scoped permissions, schema validation, no secret values |
| Credential material | OS keychain only | Re-authentication and opaque handles |
| Session/command history | Local metadata with retention setting | Sensitive-output warning, redaction, delete action |
| Recordings and logs | Opt-in files/blobs with SQLite metadata | Encryption policy, retention, preview before export |
| Temporary remote-edit files | Restricted temporary path | Short lifetime, cleanup, conflict check |
| Plugin storage | Per-plugin namespace | Quotas, permission boundary, disable without core corruption |
| Crash and diagnostic data | Minimal, opt-in where it leaves device | Preview, redaction, no raw session data by default |

## SQLite protection

SQLite is an integrity and organization layer, not automatically encryption at rest. The baseline is:

- store the database in the OS application-data directory;
- apply owner-only file permissions where supported;
- use transactions, journaling, migrations, and recovery backups;
- keep credentials out of the database;
- protect especially sensitive records with an application-level encryption design whose key is held in the OS keychain;
- evaluate an audited encrypted-SQLite option before storing high-sensitivity recordings at scale;
- never invent cryptography or derive a database key from a predictable device value.

Full-disk encryption should be detected and recommended, but it must not be treated as the only application control.

## Configuration and exports

Configuration exports are versioned, previewable, and redacted by default. They must show files and fields included before writing to a user-selected path. Import validates schema, rejects unknown privileged fields where appropriate, and never imports credential bytes.

## History, logs, and redaction

Terminal output is sensitive by default because users may paste secrets or receive tokens from commands. Recording and history are opt-in or clearly controlled, have retention settings, and offer deletion. Redaction uses known secret values and conservative pattern detection, but the UI must state that redaction is best effort.

Support bundles exclude session content unless the user explicitly selects it after a preview.

## Secure deletion

Deleting a database row or file does not guarantee physical deletion from SSDs, snapshots, backups, filesystem journals, or disk images. Prefer short retention, encrypted sensitive data, and cryptographic deletion of the encryption key where feasible. Document this limitation rather than promising a wipe the application cannot verify.

## Backups and recovery

Backups must be opt-in and clearly classified. A recovery backup should be integrity-checked, encrypted according to the sensitivity of included data, and never include plaintext credentials. Restore requires schema validation and should not overwrite active data without confirmation.
