# Secure Development and Responsible Disclosure

## Code review requirements

Every change that touches credentials, authentication, host-key verification, network connections, plugins, update paths, filesystem writes, command execution, telemetry, or sensitive data needs a security-aware reviewer. Reviewers must check:

- trust boundaries and authority;
- least-privilege impact;
- input validation and output encoding;
- secret handling and logging;
- failure and cancellation behavior;
- migration and rollback safety;
- platform differences;
- tests proving both allowed and denied behavior.

## Static and dependency analysis

CI should enforce formatting, compiler/linter warnings, Rust Clippy, strict TypeScript checks, secret scanning, dependency vulnerability audits, license checks, SBOM generation, and platform build smoke tests. Tool findings receive an owner and disposition; they are not hidden merely to make CI green.

## Security testing

Maintain tests for:

- keychain failure and access denial;
- plaintext-secret redaction and export safety;
- changed and unknown SSH host keys;
- weak algorithm rejection and scoped legacy exceptions;
- unsafe path and command inputs;
- plugin permission denial, crash, timeout, and protocol abuse;
- package signature failure and update rollback;
- malformed remote output and oversized messages;
- SQLite migration failure and backup recovery;
- port-forward bind restrictions.

Add fuzzing or property-based tests for protocol parsing, settings/config parsing, manifests, terminal metadata, and path handling where practical.

## Penetration testing

Before the first stable release, commission an independent review focused on the desktop IPC boundary, plugin isolation, credential lifecycle, update verification, SSH/host-key behavior, remote file writes, and network providers. Repeat after material changes or on a risk-based schedule. Findings need severity, owner, mitigation, regression coverage, retest evidence, and documented residual risk.

## Secure release process

Release work follows [supply-chain security](supply-chain.md) and [release strategy](../operations/release-strategy.md): protected tags, signed artifacts, provenance, SBOM, staged rollout, upgrade tests, and rollback guidance.

## Responsible disclosure

Do not publish vulnerability details in public issues. The project must publish a private reporting address or security-advisory mechanism before public stable releases. Reports should include affected version/commit, platform, reproduction steps, impact, and any proof of concept without real credentials or production targets.

Maintainers should:

1. acknowledge receipt privately;
2. reproduce and assess severity;
3. coordinate a fix and regression test;
4. notify affected users with mitigation guidance;
5. publish a coordinated advisory and credit the reporter with consent;
6. document process improvements.

## Security exceptions

Exceptions such as legacy SSH algorithms, development plugins, invalid test certificates, or verbose sensitive diagnostics must be scoped, visible, time-limited where possible, disabled by default, and owned by a maintainer. Convenience flags must not silently weaken security globally.
