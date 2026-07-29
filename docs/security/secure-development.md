# Secure Development Lifecycle and Responsible Disclosure

## Lifecycle

| Stage | Required evidence |
| --- | --- |
| Plan | User problem, v1 scope fit, data classification, trust boundaries, abuse cases, platform/privacy impact |
| Design | Owning component, typed contract, limits, failure/recovery behavior, threat-model update, ADR when triggered |
| Implement | Least privilege, safe defaults, structured concurrency, reviewed dependencies, no secret or sensitive-log leakage |
| Verify | Positive/negative tests, fuzz/property tests where appropriate, migration/rollback, platform and performance evidence |
| Release | Reviewed tag, protected build/signing, SBOM/provenance, signed artifacts and metadata, staged promotion |
| Operate | Advisory monitoring, incident response, key/dependency patch readiness, support and privacy controls |
| Retire | Deprecation, migration/export, authority revocation, data deletion, signing-key and service shutdown plan |

No phase may defer a known trust-boundary defect to final hardening.

## Code review requirements

Every change touching credentials, authentication, host-key verification,
network connections, updates, filesystem writes, command execution, terminal
parsing, recording, product analytics, diagnostics, or sensitive data needs a
security-aware reviewer. Reviewers check:

- trust boundaries and authority;
- least-privilege impact;
- input validation and output encoding;
- secret handling and logging;
- failure, timeout, cancellation, and shutdown behavior;
- migration and rollback safety;
- platform differences;
- tests proving both allowed and denied behavior.

Changes to cryptography, updater trust, release signing, credential handoff, or
raw IPC exposure require two-person review when maintainer capacity permits.
Until then they require an explicitly named owner and independent review before
stable release.

## Static and dependency analysis

CI enforces formatting, compiler and linter warnings, Rust Clippy, strict
TypeScript checks, secret scanning, dependency vulnerability audits, license
checks, generated-contract freshness, SBOM generation, and platform build smoke
tests. Findings receive an owner and disposition; they are not hidden merely to
make CI green.

CI workflows:

- declare least-privilege token permissions;
- pin third-party actions to reviewed full commit SHAs;
- expose no release or signing secret to pull-request jobs;
- separate untrusted build/test jobs from protected signing/publishing jobs;
- retain auditable build inputs, artifacts, and provenance;
- use only synthetic, bounded hostile fixtures;
- perform no release-time dependency fetch outside the pinned build plan.

## Security testing

Maintain tests for:

- credential-store failure, access denial, and secret lease misuse;
- plaintext-secret redaction and export safety;
- changed, revoked, ambiguous, and unknown SSH host keys;
- weak algorithm rejection and scoped legacy exceptions;
- unsafe local/remote paths and command arguments;
- terminal escape-sequence, URI, clipboard, and shell-integration abuse;
- snippet/history control-character rejection and proof that reuse never
  synthesizes command submission;
- SFTP/SCP interruption, hostile names, overwrite, symlink, and
  legacy-protocol refusal;
- update signature failure, replay, expiry, target mismatch, key rotation, and
  rollback;
- malformed SSH configuration, themes, remote output, and oversized messages;
- SQLite migration failure and encrypted backup recovery;
- encrypted database/blob wrong-key, tamper, and plaintext-canary checks;
- port-forward bind restrictions and orphan cleanup;
- theme attempts to hide or imitate trusted safety UI.

Add fuzzing or property tests for IPC and event parsing, SFTP packets,
settings/SSH-config formats, theme tokens, terminal metadata, encrypted blob
framing, workspace layout trees, and path handling where practical.

## Security gates by phase

- **Phase 1:** restrictive content policy, explicit Tauri capabilities, no
  generic privileged command, dependency and secret scanning.
- **Phase 2:** PTY/process-tree cleanup, hostile terminal streams,
  backpressure, and bounded-memory tests.
- **Phase 3:** credential-store denial, encryption-at-rest inspection,
  migration, corruption, backup, and export tests.
- **Phases 4–6:** host-key, askpass, SSH config, SFTP/SCP, remote path,
  transfer, and listener negative tests.
- **Phase 7:** recording/history opt-in, quota, deletion, indexing, redaction,
  and plaintext-canary tests.
- **Phase 8:** hostile theme, safety-chrome invariants, accessibility, and
  shortcut-conflict tests.
- **Phase 9:** independent review, resolved critical/high findings, signing-key
  recovery exercise, provenance, and signed-artifact verification.

## Independent security review

Before stable release, commission an independent review focused on desktop IPC,
credential lifecycle, update verification, SSH and host-key behavior, SFTP/SCP
paths and writes, port binding, terminal parsing, recording retention, encrypted
profile/key management, and the release pipeline. Repeat after material
boundary changes or on a risk-based schedule.

Findings need severity, owner, mitigation, regression coverage, retest evidence,
and documented residual risk.

## Secure release process

Release work follows [supply-chain security](supply-chain.md) and
[release strategy](../operations/release-strategy.md): protected tags, signed
artifacts, provenance, SBOM, staged rollout, upgrade tests, and rollback
guidance.

## Responsible disclosure

Do not publish vulnerability details in public issues. Before public stable
releases, publish a private reporting address or security-advisory mechanism
with named ownership. Reports should include affected version or commit,
platform, reproduction steps, impact, and a proof of concept that uses no real
credentials or production target.

Maintainers:

1. acknowledge receipt privately;
2. reproduce and assess severity;
3. coordinate a fix and regression test;
4. notify affected users with mitigation guidance;
5. publish a coordinated advisory and credit the reporter with consent;
6. document process improvements.

## Security exceptions

Exceptions such as legacy SSH algorithms, invalid test certificates, or verbose
sensitive diagnostics must be scoped, visible, time-limited where possible,
disabled by default, and owned by a maintainer. Convenience flags must not
weaken security globally. Legacy SCP has no v1 exception.
