# ADR-009: Signed direct updates with explicit rollback protection

- Status: Accepted
- Date: 2026-07-29
- Owners: Relio maintainers

## Context

Relio is an infrastructure application. A compromised update can reach user
credentials and production systems. TLS alone does not prove artifact
authenticity, and an artifact signature alone does not provide channel,
platform, freshness, migration, or rollback policy.

The project is initially maintained by a small team and uses one authoritative
update origin.

## Decision

Direct-download builds use a Rust-owned Tauri updater path with:

- an embedded update verification public key;
- signed metadata binding artifact digest, length, version, channel, platform,
  architecture, package type, and expiry;
- monotonic metadata/version checks in the normal path;
- OS code signing/notarization where supported;
- protected staging, pre-migration encrypted backup, health check, and rollback.

The webview cannot set update endpoints or keys and cannot install arbitrary
artifacts. Distribution-managed Linux packages use the distribution updater.

The first release will not implement full TUF. A future multi-origin or
delegated update model must reevaluate that decision.

## Rationale

This provides a realistic secure baseline using the chosen desktop framework
while keeping update authority in the core. It avoids introducing server and
key-role complexity before there is an operational team to maintain it.

## Alternatives considered

- **TLS and checksum only:** rejected because the publishing endpoint could
  replace both.
- **OS signature only:** rejected because it does not encode Relio channel and
  metadata policy consistently across platforms.
- **TUF immediately:** deferred as disproportionate to a single-origin initial
  release, while acknowledging its stronger compromise/freeze model.
- **No updater:** rejected for direct downloads because delayed security updates
  create material risk.

## Consequences

- The update key is high impact and needs protected ownership, rotation, and
  emergency procedures.
- Planned rotation must be introduced by an already trusted release.
- Compromise of the sole trusted key may require an out-of-band manual recovery
  installer.
- Application rollback and data rollback are separate because migrations are
  forward-only.

## Migration or follow-up

Before stable release, rehearse key loss/compromise, rotation, staged rollout,
database rollback, and platform-specific install recovery. See
[update security](../../security/updates.md).

## Evidence and references

- [Tauri capability security boundaries](https://v2.tauri.app/security/capabilities/)
