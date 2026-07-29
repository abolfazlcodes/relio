# Update Security

## Trust objective

An update must be authentic for Relio, intended for the selected channel,
platform, architecture, and current installation, newer than the installed
version unless an explicit recovery flow is used, and safe to stage without
destroying the known-good application or user profile.

HTTPS provides transport protection but is not artifact authenticity.

## Update model

Direct-download builds use the Tauri updater only through a Rust-owned update
service. The webview may request a check and display state; it cannot provide an
endpoint, public key, artifact path, or unverified metadata and cannot invoke a
generic install command.

Required verification layers:

1. validated TLS to a compiled allowlist of update origins;
2. signed update metadata and artifact signature verified against a public key
   embedded in the installed application;
3. exact channel, version, platform, architecture, package type, length, and
   cryptographic digest match;
4. operating-system code signature/notarization verification where supported;
5. migration and free-space preflight before replacement.

No layer disables another. A signature failure, metadata mismatch, redirect
outside the allowlist, or unsupported downgrade fails closed.

## Metadata and rollback protection

Update metadata includes:

- application and contract version;
- release channel;
- target OS, architecture, and package format;
- artifact length and digest;
- artifact signature;
- publication and expiry time;
- minimum updatable version;
- required data schema migration range;
- release-notes location;
- rollout identifier.

The client persists the highest trusted version and metadata sequence per
channel. It rejects older metadata or artifacts in the normal path. Expired
metadata produces a visible inability to prove freshness; the app continues to
work locally.

The first release does not implement the full Update Framework (TUF). That is a
deliberate complexity decision, not a claim that one signing key handles every
freeze, rollback, or key-compromise scenario. Any move to multiple update
origins requires a new ADR and should evaluate TUF at that time.

## Download and staging

- Update checks follow the user's channel and network settings; stable builds do
  not silently switch channel.
- Download to a protected staging directory with a random name.
- Stream length and digest verification and enforce a maximum artifact size.
- Do not execute, mount, preview, or inspect unverified package content through
  the webview.
- Verify completely before presenting install readiness.
- Keep the active binary and previous known-good installer/package until the new
  version launches and passes startup health checks.
- Clean failed or abandoned staged artifacts after a bounded period.

Automatic background download may be opt-in. Installation requires a clear user
action except for a future organization-managed policy designed separately.

## Data migration and application rollback

Before installation:

- confirm database-key access;
- verify profile integrity;
- create a bounded encrypted pre-migration backup when the release changes the
  schema;
- verify disk-space reserve;
- stop or obtain explicit handling for active sessions, transfers, tunnels, and
  pending remote edits.

Application rollback cannot open a newer incompatible schema. The recovery flow
offers the previous application plus restoration of the pre-update encrypted
backup and states that post-update changes will be lost.

## Key management

- Update private keys never live in the repository, release artifact, ordinary
  CI variable available to pull requests, or developer workstation.
- Signing occurs in a protected release environment with named maintainers,
  least-privilege access, audit logs, and recovery ownership.
- The public verification key is embedded in the application and changes only
  in a release signed by a currently trusted key.
- Planned rotation ships trust in the next key before the old key is retired.
- Emergency compromise of the only trusted key may require disabling automatic
  updates and directing users to an out-of-band, OS-signed manual installer.
  Documentation must not pretend this can be solved by signing a new key with
  the compromised key.
- Key revocation, loss, rotation, and maintainer departure are rehearsed before
  stable release.

## Platform ownership

- Windows and macOS direct-download artifacts use Relio update signatures and OS
  signing/notarization.
- Linux distribution-managed packages update only through the distribution.
- A directly downloaded Linux artifact may use Relio's updater only when its
  package format supports safe replacement and rollback.
- Unsupported package managers receive notification with the authoritative
  release location, not an attempted self-modification.

## Rollout and incident response

- Promote nightly to preview to stable; do not rebuild the same version between
  channels.
- Use staged rollout metadata without giving the server authority to serve an
  unsigned artifact.
- Keep update failure diagnostics local and available for previewed export; the
  client does not report update success or failure automatically.
- A suspected signing or update compromise stops publication, freezes promotion,
  preserves evidence, publishes an advisory through an independent channel, and
  follows the key-compromise runbook.

## Required tests

- valid update, wrong key, corrupt signature, digest/length mismatch;
- wrong channel/platform/architecture and redirect to unapproved origin;
- replayed, expired, and lower-version metadata;
- interrupted download, disk full, cancellation, and staged-file tampering;
- OS signature failure;
- database migration failure and application/data rollback;
- key rotation overlap and revoked-key behavior;
- updater unavailable while all local core features continue to work.
