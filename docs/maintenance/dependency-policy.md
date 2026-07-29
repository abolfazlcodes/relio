# Dependency Lifecycle Policy

## Principle

Every dependency becomes maintenance work: advisories, APIs, transitive code,
licenses, build time, platform failures, and eventual replacement. Relio
accepts a dependency when its total lifetime cost is lower than owning the
needed behavior and when it does not weaken a trust boundary.

## Adoption record

A production dependency proposal must state:

- exact capability and owning module;
- why the standard library, OS facility, existing dependency, or small
  project-owned implementation is insufficient;
- maintenance activity, release/support policy, advisory history, and bus
  factor;
- direct and transitive licenses;
- native code, `unsafe`, build scripts, lifecycle scripts, network access, and
  generated artifacts;
- binary size, startup, memory, build-time, and platform impact where relevant;
- trust-boundary and data-access implications;
- test seam and removal/replacement plan;
- accountable owner.

Security-critical choices require an ADR and proof on every Tier 1 target.
Developer-only dependencies still require license, script, and supply-chain
review but may use a shorter record.

## Dependency classes

| Class | Examples | Review cadence |
| --- | --- | --- |
| Critical runtime | Tauri, webview bridge, SQLCipher, crypto, terminal parser, OpenSSH interaction, updater | Monthly advisory review; quarterly health review |
| Runtime | React, state/query utilities, UI primitives | Quarterly |
| Build/release | bundlers, package managers, CI actions, signing tools | Each release and quarterly |
| Development/test | test runners, linters, fixture tools | Twice yearly or on advisory |

Every direct dependency has an owner and class in a maintained inventory.
Transitive dependencies are tracked through lockfiles and SBOM, not manually
assigned one by one.

## Version policy

- Pin toolchains, package manager, CI actions, lockfiles, and release inputs.
- Use the narrowest range compatible with reproducible security updates.
- Do not combine dependency upgrades with unrelated feature work.
- Upgrade critical dependencies one logical group at a time with compatibility,
  performance, migration, and rollback evidence.
- Do not automatically merge a major upgrade or any update that changes native
  code, build scripts, permissions, protocol behavior, or cryptography.
- Maintain one supported dependency line unless a platform constraint is
  documented with an owner and removal date.

Automated update tools may open proposals and provide evidence; they do not
replace maintainer review.

## Rust-specific rules

- Default features are reviewed rather than accepted automatically.
- New `unsafe` or native bindings require focused ownership and tests.
- Crates that expose secrets, raw paths, subprocesses, sockets, or
  serialization are reviewed as boundary code.
- Feature flags represent real target/build capabilities, not arbitrary product
  combinations. Unsupported combinations do not accumulate.
- Duplicate major versions of large or security-critical crates require a
  documented temporary exception.

## Frontend-specific rules

- Prefer platform and React primitives before adding wrapper packages.
- UI libraries must permit semantic markup, accessibility repair, tree-shaking,
  and design-token ownership.
- State libraries must not become a second domain layer or receive terminal
  bytes.
- Packages with install scripts, native binaries, remote asset loading,
  telemetry, or broad browser APIs require explicit security review.
- Avoid single-component “micro-dependencies” when the behavior is clearer to
  own locally.

## Tauri and platform dependencies

Only required Tauri plugins/capabilities are enabled. Adding one must document
the exact commands, window scope, platform behavior, and negative capability
tests. Prefer maintained OS APIs behind Relio-owned ports over leaking
framework types into domain/application modules.

## Vulnerability and abandonment response

Advisories are triaged by reachability, boundary crossed, exploit preconditions,
and affected releases. Critical reachable issues receive immediate mitigation
or release blocking. An accepted risk has an owner, rationale, user impact,
expiry, and removal condition.

A dependency enters replacement review when it is unmaintained, repeatedly
breaks supported platforms, cannot meet security policy, blocks toolchain
updates, changes license incompatibly, or costs more to adapt than replace.
Adapters and contract tests are the migration seam; carrying two
implementations is temporary and has an expiry.

## Removal

Removal includes direct and transitive package cleanup, capability/config
removal, lockfile/SBOM update, migration of persisted identifiers if any,
documentation cleanup, and verification that release artifacts contain no
obsolete native component.
