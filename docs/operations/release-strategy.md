# Release Strategy

## Release channels

- **Nightly:** automated builds for contributors; may be unstable and must not be recommended for sensitive production operations.
- **Preview:** manually promoted builds for early adopters and compatibility testing.
- **Stable:** signed, documented releases with migration and rollback guidance.

The exact cadence should follow project capacity. Predictable quality is more important than frequent version numbers.

## Build and artifact requirements

Every distributable build should record:

- source commit and version;
- target operating system and architecture;
- Rust, frontend, and toolchain versions;
- dependency lockfile state;
- build configuration;
- checksums and signature metadata.

Builds should be reproducible enough that maintainers can investigate a published artifact. Signing keys and release credentials never live in the repository.

## Promotion gates

Promote a build only when:

1. supported platform smoke tests pass;
2. migration and upgrade tests pass from the previous stable version;
3. security-sensitive tests pass;
4. known regressions are documented;
5. release notes explain user-visible changes, risks, and rollback;
6. the artifact is signed and its checksum is published.

## Incident response

If a release causes data loss, credential exposure, unsafe connection behavior, or widespread crashes:

- stop promotion;
- publish a clear advisory and affected versions;
- provide a safe rollback or mitigation;
- preserve relevant diagnostics without requesting sensitive session data;
- create a regression test;
- document the root cause and process improvement.

## Open-source governance before first public release

Before shipping public binaries, resolve the project license, supported platforms, maintainer ownership, security contact, signing ownership, privacy/telemetry policy, and third-party attribution process.
