# Release Strategy

The normative build graph, isolation boundaries, artifact identity, and
promotion model are in
[implementation architecture](../architecture/implementation-architecture.md#20-build-architecture).

## Release channels

- **Nightly:** automated builds for contributors; may be unstable and must not be recommended for sensitive production operations.
- **Preview:** manually promoted builds for early adopters and compatibility testing.
- **Stable:** signed, documented releases with migration and rollback guidance.

The exact cadence should follow project capacity. Predictable quality is more important than frequent version numbers.

Published maintenance windows and platform lifecycle rules are defined in the
[compatibility and support policy](../maintenance/compatibility-policy.md). A
release cadence must not create more supported lines than maintainers can patch,
build, sign, test, and recover.

## Build and artifact requirements

Every distributable build should record:

- source commit and version;
- target operating system and architecture;
- Rust, frontend, and toolchain versions;
- dependency lockfile state;
- build configuration;
- checksums and signature metadata;
- SBOM and provenance identity;
- encrypted-data, workspace-export, and theme-schema compatibility;
- update channel, platform, architecture, and package type.

Builds should be reproducible enough that maintainers can investigate a published artifact. Signing keys and release credentials never live in the repository.

## Promotion gates

Promote a build only when:

1. supported platform smoke tests pass;
2. migration and upgrade tests pass from the previous stable version;
3. security-sensitive tests pass;
4. known regressions are documented;
5. release notes explain user-visible changes, risks, and rollback;
6. the artifact is signed and its checksum is published;
7. updater metadata binds the exact artifact, target, channel, version, length,
   digest, and expiry;
8. performance budgets pass on the reference systems;
9. unresolved security/privacy findings are explicitly release-blocking or have
   a documented owner, expiry, and accepted residual risk.

Tier 1 platform failure blocks promotion. Tier 2 status is published. The
support policy is in [platform support](../architecture/platform-support.md).

## Incident response

If a release causes data loss, credential exposure, unsafe connection behavior, or widespread crashes:

- stop promotion;
- publish a clear advisory and affected versions;
- provide a safe rollback or mitigation;
- preserve relevant diagnostics without requesting sensitive session data;
- create a regression test;
- document the root cause and process improvement;
- assess signing/update key compromise and disable automatic promotion when
  authenticity is uncertain.

Application rollback and data rollback are separate. A previous application
cannot write a newer incompatible database schema; restoration uses the
explicit encrypted pre-migration backup and discloses loss of post-update
changes.

## Open-source governance before first public release

Before accepting implementation dependencies or publishing source, resolve the
project license and attribution policy. Before shipping public binaries, also
resolve supported platforms, maintainer ownership, a private security contact,
application/update/platform signing ownership and recovery, privacy policy, and
third-party notices.

The update trust and key lifecycle requirements are in
[update security](../security/updates.md).
