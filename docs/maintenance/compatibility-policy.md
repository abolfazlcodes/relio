# Compatibility and Support Policy

## Support dimensions

Relio separately versions and supports:

- application releases;
- operating systems, architectures, webviews, and OpenSSH providers;
- encrypted database and blob formats;
- workspace/theme/export formats;
- IPC and generated frontend contracts;
- update and installer paths.

“Supported” means tested in the published matrix with an owner and a response
path. “May work” is not a support promise.

## Release support

Before 1.0, the latest preview is the only feature-development line, but
security and data-loss fixes are provided for the latest published preview when
practical. After 1.0:

- the latest minor release receives fixes;
- the previous minor receives critical security/data-loss fixes for at least
  90 days after the next minor release;
- a superseded patch release receives no separate support once its replacement
  is available;
- longer support requires a separately staffed LTS policy and is not implied.

Security advisories state exact affected and fixed versions. Support windows may
be extended, never shortened retroactively without a public notice and
mitigation.

## Upgrade and data compatibility

Every stable release upgrades directly from the previous stable minor. At least
the two previous minor data-schema versions remain readable through tested
migrations. Older profiles require stepping through an archived supported
release or an explicitly tested migration tool; Relio does not accumulate
unbounded migration code in the main startup path.

Export and recovery formats publish their reader window. A writer never emits a
new format under an old version number. Downgrade is not assumed: it uses the
documented pre-update encrypted backup when schema compatibility is absent.

Internal desktop IPC needs compatibility only within one packaged application,
but schema changes remain versioned to protect generated bindings, tests, crash
diagnostics, and rolling development. It is not a remote public API.

## Platform lifecycle

Tier changes require evidence, release notes, and at least one minor-release
notice when practical. Relio follows supported vendor OS/webview/toolchain
lines; it does not promise operation on an OS that no longer receives security
updates. A platform may be downgraded when required signing, webview, keychain,
OpenSSH, encrypted database, or test infrastructure cannot meet security gates.

Dropping a platform requires:

- usage/support evidence where privacy permits collection;
- security and maintenance analysis;
- export/recovery guidance;
- final known-working version;
- removal date and affected package/update behavior.

## Deprecation

A deprecation names the replacement, first deprecated release, earliest removal
release/date, migration, and owner. Security-unsafe behavior may be removed
faster, with advisory and mitigation. Feature flags and legacy exceptions do
not become permanent compatibility promises.

## Support boundaries

Maintainers support Relio behavior, documented provider ranges, and synthetic
reproductions. They do not request production credentials, guarantee a remote
system is safe, debug arbitrary custom shell configuration without a minimal
reproduction, or promise support for undocumented SSH directives and package
managers.
