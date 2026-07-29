# Threat Model

## Scope and assumptions

This model covers the Relio desktop application, its local data, installed plugins, update/package paths, network transports, and integrations with local and remote systems. It assumes the user may operate production systems and that a compromise can have business or operational impact.

The operating system, its kernel, the user’s hardware, and the cryptographic implementations supplied by the platform are trusted dependencies but not assumed to be perfect. If the device is fully compromised, Relio cannot guarantee secrecy of data while it is being used.

## Assets we protect

| Asset | Confidentiality | Integrity | Availability | Examples |
| --- | --- | --- | --- | --- |
| Credentials | Critical | Critical | High | passwords, private keys, tokens, certificates, agent references |
| Host identity data | High | Critical | High | aliases, addresses, usernames, jump hosts, known-hosts records |
| Remote operations | High | Critical | Critical | commands, uploads, tunnels, service actions |
| Local files | High | High | High | workspace files, downloaded configs, temporary edit buffers |
| Session content | High | High | Medium | terminal input/output, command history, recordings, logs |
| Workspace metadata | Medium to high | High | High | projects, environments, snippets, layouts, infrastructure inventory |
| Plugin and update artifacts | Medium | Critical | High | packages, manifests, signatures, dependency metadata |
| Privacy and diagnostics | High | High | Medium | crash data, connection metadata, telemetry decisions |

## Attackers considered

- malicious local applications attempting to read files, keychain data, IPC, or process memory;
- compromised or malicious plugins;
- malware or ransomware on the device;
- a thief with a stolen or unlocked device;
- malicious insiders with access to a plugin, build pipeline, release account, or support process;
- compromised dependencies, registries, build tools, or package maintainers;
- supply-chain attackers tampering with source, artifacts, update metadata, or marketplace packages;
- network attackers attempting MITM, downgrade, DNS, proxy, or tunnel abuse;
- compromised remote hosts returning malicious output or attempting to abuse forwarding and file operations.

## Trust boundaries

1. React/webview to Rust core through typed IPC.
2. Rust core to OS keychain and filesystem.
3. Rust core to plugin host through capability-scoped IPC.
4. Relio to local processes and shells.
5. Relio to remote SSH, TLS, proxy, and forwarding endpoints.
6. Release pipeline to signed installers, packages, and update metadata.
7. Optional AI or sync providers to approved user context.

## Attack scenarios and mitigations

| Scenario | Impact | Primary mitigations | Remaining risk |
| --- | --- | --- | --- |
| Malicious plugin requests credentials or arbitrary command execution | Credential theft or production impact | Out-of-process runtime, explicit capabilities, no raw credential API, user-visible consent, timeouts, audit events | A user may approve an overly broad capability; process isolation is not a complete sandbox |
| Local malware reads Relio config or SQLite files | Host and workflow disclosure | OS ACLs, no plaintext secrets, keychain references, optional encrypted sensitive records, redacted exports | Malware running as the user may still read accessible data and observe active sessions |
| Stolen unlocked device is used to open Relio | Unauthorized operations | OS session lock awareness where available, re-authentication for credential use and sensitive actions, keychain access controls, workspace lock option | An attacker with an active OS session can use resources available to that session |
| Host-key change is accepted for convenience | MITM and credential exposure | Strict known-host verification, fingerprint display, explicit changed-key block, no silent replacement | Users may override warnings; operational recovery must remain understandable |
| Weak SSH algorithm is negotiated | Reduced confidentiality or downgrade | Curated algorithm policy, weak algorithms disabled by default, warning and time-limited override for legacy hosts | Legacy environments may need exceptions and become a residual risk |
| Plugin or dependency package is replaced in transit | Code execution or data theft | Signed packages, integrity checks, pinned/locked dependencies, provenance, isolated runtime, update rollback | Signing-key compromise or a trusted publisher compromise remains possible |
| Terminal output contains a password and is recorded | Secret disclosure | Recording opt-in, sensitive-output warnings, redaction heuristics, retention controls, encrypted storage policy | Perfect detection is impossible; users can intentionally or accidentally expose secrets |
| Unsafe path or command composition is exploited | Local or remote unauthorized write/execute | Structured arguments, path validation, explicit target/scope confirmation, no shell concatenation | A legitimate user can still authorize a destructive action |
| Port forwarding exposes a production service | Network exposure | Loopback default, bind-address warning, visible tunnel lifecycle, privileged-port checks, stop controls | A user can intentionally bind broadly; the UI cannot know all network consequences |
| Compromised remote host injects UI-like instructions in output | Social engineering or unsafe action | Remote output treated as untrusted data, no automatic action parsing, explicit execution boundary | Users may follow malicious text manually |
| Update pipeline is compromised | Broad user compromise | Signed artifacts, protected release credentials, reproducible builds, SBOM, staged rollout, rollback | A validly signed malicious release remains a high-impact failure |

## Security objectives

- secrets are not written to ordinary application data;
- security-sensitive operations have an explicit target and consent boundary;
- untrusted extensions and remote output cannot directly invoke privileged core behavior;
- host identity and TLS certificates are verified rather than inferred from convenience;
- local data is protected by platform controls and a documented encryption strategy;
- a compromised component has the smallest practical blast radius;
- failures are visible, diagnosable, and fail closed where possible.

## Residual risks accepted for the initial architecture

- a fully compromised OS can observe input, memory, windows, and network activity;
- plugin process isolation is weaker than a hardened OS sandbox on some platforms;
- terminal recordings and logs can contain secrets that automated redaction misses;
- user-approved commands can still cause damage;
- availability depends on local OS facilities, remote hosts, and network paths;
- cross-platform security parity requires platform-specific testing and may not be perfect.

Residual risks must be reviewed before each stable release and documented in release notes when user action or platform configuration is required.
