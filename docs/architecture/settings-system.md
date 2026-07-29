# Settings System

## Goals

Settings should be discoverable in the UI, inspectable as data, safe to migrate,
and composable across global, workspace, host, and session scopes.

## Scope precedence

The effective value is resolved from lowest to highest ordinary scope:

```text
built-in default
  -> user setting
  -> workspace setting
  -> host setting
  -> session override
```

An application safety policy constrains the final value and cannot be weakened
by an ordinary scope. Only settings that declare a scope may be written at that
scope.

Precedence is evaluated in a concrete context. A host override follows a
workspace override because it describes the selected target; a session override
is ephemeral unless its schema explicitly supports persistence.

## Schema requirements

Every setting has:

- stable key and type;
- human-readable title and description;
- default value;
- allowed scopes;
- validation, size, and normalization rules;
- merge behavior (`replace` by default; no implicit deep merge);
- sensitivity classification;
- deprecation and migration metadata;
- whether changing it requires a restart or session reconnect.
- whether safety policy can further constrain it.

## Storage

Non-secret settings are stored in the encrypted profile database through the
settings service. There is no setting type that stores secret bytes. A setting
may store only an opaque secret handle whose purpose and ownership are validated
again at use time.

A settings export includes a schema version and previewed non-secret values.
Secret handles are always excluded. Same-profile recovery uses the separate
encrypted profile backup flow rather than a settings export.

## UX rules

- show the effective value and its source scope;
- explain when a setting is overridden;
- provide reset-at-this-scope rather than only a global reset;
- validate before saving and preserve the last valid value;
- make experimental settings visibly experimental;
- provide an equivalent searchable command-palette action for important settings.
- distinguish an inherited value, explicit value, and policy-constrained value;
- show when a change affects active sessions or requires reconnect.
