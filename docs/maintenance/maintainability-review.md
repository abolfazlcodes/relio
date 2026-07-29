# Ten-Year Maintainability Review

## Verdict

Relio’s focused v1 scope and trusted-core architecture are sustainable. The
largest risks are operational: concentrating ownership, allowing dependency and
platform promises to grow without capacity, duplicating normative
documentation, and retaining compatibility/test machinery after its value
expires. The policies in this section make those costs explicit before code
exists.

Review completed against every repository document on 2026-07-29.

## Findings and dispositions

| Area | Two-year failure mode | Disposition |
| --- | --- | --- |
| Repository organization | Premature crate/package explosion or one desktop module becoming a monolith | Keep cohesive initial modules; extract only at documented process/security/reuse/build triggers |
| Ownership | Critical subsystems and releases depend on one maintainer | Define roles, area ownership, backups, inactivity transfer, and two-person stable-release gate |
| Decision history | ADRs 001–006 embedded while later ADRs use files | Preserve history, add one canonical ADR index, require one file from ADR-010 onward |
| Documentation | Multiple documents restate the same contract and drift | Define sources of truth, precedence, owner/review metadata, review cadence, and controlled decomposition |
| Naming | “Project,” “workspace,” “session,” and “connection” become interchangeable | Publish canonical glossary and identifier conventions |
| Architecture | Application services, adapters, and feature packages accumulate bypasses | Keep enforced dependency direction, composition root, generated contracts, and extraction criteria |
| Canonical architecture | The implementation contract grows into an unreviewable monolith | Split only when real owners/cadences diverge; retain the old path as a stable index |
| Frontend | Global state and third-party packages become a second core | Keep feature controllers, authority caches, terminal isolation, dependency adoption tests, and package limits |
| Rust | Trait/crate proliferation hides ownership; feature combinations multiply | Traits live with consumers, crates need extraction evidence, and feature flags represent supported build capabilities only |
| Tauri | Convenience plugins silently broaden IPC and permissions | Treat every plugin/capability as a security-sensitive direct dependency with scoped negative tests |
| Terminal | xterm.js addons gain access to terminal content/input without review | Treat each addon as an independently reviewed, pinned direct dependency |
| Persistence | “SQLCipher-compatible” permits an incidental provider choice | Require a Phase 3 ADR for provider, binding, crypto backend, build, license, and update strategy |
| OpenSSH | Provider/version/platform behavior expands beyond test capacity | Publish diagnosed provider ranges and capability degradation; native SSH requires measured justification |
| Dependencies | Automated upgrades, duplicate versions, build scripts, and abandonment accumulate | Add classes, owners, cadence, adoption records, risk expiry, upgrade isolation, and removal completion |
| Platform support | “Cross-platform” becomes an unlimited distribution promise | Maintain explicit tiers, evidence, lifecycle notice, and a supported-versus-may-work distinction |
| Compatibility | Migration readers and old formats remain forever | Define tested reader windows, step-up migration for older profiles, deprecation, and bounded supported release lines |
| Testing | Slow/flaky suites become ritual and contributors cannot diagnose CI | Require suite ownership, purpose, runtime, triage, fixture provenance, expiry, and layered CI lanes |
| Fixtures | Large hostile/protocol data contains secrets or unclear provenance | Require synthetic provenance, limits, owner, license, protected behavior, and retirement condition |
| Releases | Each channel is rebuilt or creates another patch obligation | Promote immutable artifacts and limit supported release lines to maintainer capacity |
| Security response | Placeholder private channel and single signing operator fail during incidents | Make named contact and independent release/security recovery explicit blockers |
| Contribution | Uncoordinated large PRs impose irreversible review/support cost | Require ready issue/owner contact, draft boundary review, dependency evidence, and compatibility impact |
| Roadmap | “Done” means shipped but not owned | Add ownership, support, dependency, runbook, rollback, and fixture maintenance gates |
| Future scalability | Plugin/cloud/AI abstractions leak into v1 despite being deferred | Preserve zero v1 APIs, schemas, permissions, or packages for future ideas; revisit through ADR and threat model |

## Complexity budget

Relio should spend complexity only where it buys one of:

- a trust or process boundary;
- observable user value in the active milestone;
- a tested compatibility seam;
- measurable performance or platform isolation;
- safer recovery or reduced operational toil.

The following require explicit justification:

- a new runtime process, Rust crate, TypeScript package, global store, Tauri
  plugin, feature flag, persisted schema, public format, background worker,
  build target, release channel, supported platform, or dependency with native
  code/build scripts;
- a second implementation of an adapter;
- a compatibility path older than the published support window.

The pull request states what existing complexity is removed or why the new cost
is durable. “We may need it later” is insufficient.

## Re-review triggers

Repeat this review before Phase 1, first preview, 1.0, plugin/runtime
extensibility design, adding a hosted service, changing SSH or database
provider, adding a long-term support line, or when two consecutive releases
miss their quality gates because of maintainer capacity.
