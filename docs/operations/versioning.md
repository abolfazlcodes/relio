# Versioning Strategy

## Application versions

Use Semantic Versioning for the application:

- **MAJOR:** incompatible public API, user data/export format, or documented
  behavior change without a compatible migration;
- **MINOR:** backward-compatible capability or feature addition;
- **PATCH:** backward-compatible bug fix or security fix.

An ordinary backward-compatible internal database migration does not by itself
require an application major version. A migration that removes user-visible
behavior, breaks export/public contracts, or cannot meet the documented
upgrade/rollback window is a major change.

Pre-1.0 versions may change faster, but every breaking change still needs migration notes and a clear compatibility statement.

## Contract versions

Desktop IPC DTOs, theme schema, settings schema, encrypted blob/backup formats,
workspace export, recording format, and database schema each have an explicit
version where persisted or cross-process compatibility requires it.
Application version alone is not enough to determine compatibility.

Readers should tolerate additive fields where practical. Removing or changing
meaning requires a new version or a compatibility adapter.

## Data migrations

Migrations are ordered and every schema change has an upgrade test from the
oldest supported version. A migration is either transactionally retryable or
records an explicit resumable state; “idempotent” is not assumed automatically.
A failed migration fails safely with an encrypted known-good backup.

Migrations are forward-only. Application rollback may require restoring the
pre-update backup and losing changes made by the newer version; the recovery UI
states this before proceeding.

## Deprecation

Deprecate public APIs with:

- replacement guidance;
- first version deprecated;
- earliest removal version;
- runtime or build-time diagnostics;
- documentation and migration example.
