# Versioning Strategy

## Application versions

Use Semantic Versioning for the application and public plugin contracts:

- **MAJOR:** incompatible public API, data migration, or plugin contract change;
- **MINOR:** backward-compatible capability or feature addition;
- **PATCH:** backward-compatible bug fix or security fix.

Pre-1.0 versions may change faster, but every breaking change still needs migration notes and a clear compatibility statement.

## Contract versions

IPC, plugin protocol, theme schema, settings schema, and export formats each have an explicit version. Application version alone is not enough to determine compatibility.

Clients and plugins should tolerate additive fields where practical. Removing or changing meaning requires a new version or a compatibility adapter.

## Data migrations

Migrations are ordered, idempotence is considered explicitly, and every schema change has an upgrade test from the oldest supported version. A failed migration must fail safely with a recoverable backup path.

## Deprecation

Deprecate public APIs with:

- replacement guidance;
- first version deprecated;
- earliest removal version;
- runtime or build-time diagnostics;
- documentation and migration example.

## Plugin compatibility

The plugin manifest declares the supported host API range. The host does not activate a plugin outside its declared range unless the user explicitly runs a compatibility override with a warning.
