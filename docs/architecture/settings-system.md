# Settings System

## Goals

Settings should be discoverable in the UI, inspectable as data, safe to migrate, and composable across global, workspace, host, session, and plugin scopes.

## Scope precedence

The effective value is resolved from lowest to highest scope:

```text
built-in default
  -> user setting
  -> workspace setting
  -> host setting
  -> session override
```

Only settings that declare a scope may be written at that scope. A workspace must not silently override a security-sensitive global policy.

## Schema requirements

Every setting has:

- stable key and type;
- human-readable title and description;
- default value;
- allowed scopes;
- validation and normalization rules;
- sensitivity classification;
- deprecation and migration metadata;
- whether changing it requires a restart or session reconnect.

Plugins use namespaced keys and cannot shadow core settings.

## Storage

Non-secret settings are stored as versioned local data, backed by SQLite once persistence is implemented. Secret settings contain only an opaque credential-store reference. A settings export includes schema version and non-secret values, with an explicit option to include references but never secret material.

## UX rules

- show the effective value and its source scope;
- explain when a setting is overridden;
- provide reset-at-this-scope rather than only a global reset;
- validate before saving and preserve the last valid value;
- make experimental settings visibly experimental;
- provide an equivalent searchable command-palette action for important settings.

## Synchronization boundary

Settings sync is a provider that reads and writes an allowlisted, non-secret settings document. Sync must expose conflicts, preserve local data, and never become required for startup. Credentials, host private keys, session output, and recordings are excluded unless a future design explicitly introduces end-to-end encrypted handling.
