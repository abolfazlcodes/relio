# Threat Model

## Scope and assumptions

This model covers the Relio desktop application, local data, application/update
paths, SSH/SFTP/SCP transports, port forwarding, local processes, imported
workspaces and themes, and interactions with remote systems. Users may operate
production systems, so compromise can have operational and business impact.

The operating system, kernel, hardware, and platform cryptographic
implementations are trusted dependencies but not assumed perfect. A fully
compromised device can observe data while Relio uses it.

The Rust core and protected release-signing process are trusted. The webview is
less trusted. Imported data, theme data, terminal output, remote systems,
network paths, update transport, user-selected files, and diagnostics are
untrusted.

This model uses asset and trust-boundary analysis informed by STRIDE. Risk is
assessed by impact, exploitability, exposure, and detectability. The table is a
design tool, not proof of security.

## Assets

| Asset | Confidentiality | Integrity | Availability | Examples |
| --- | --- | --- | --- | --- |
| Credentials | Critical | Critical | High | passwords, private keys, passphrases, certificates, agent references |
| Host identity | High | Critical | High | addresses, usernames, jump hosts, known-host records |
| Remote operations | High | Critical | Critical | commands, uploads, downloads, tunnels |
| Local/remote files | High | High | High | SSH configuration, referenced key files, downloaded files, edit buffers |
| Session content | High | High | Medium | input/output, history, recordings, logs |
| Workspace metadata | Medium to high | High | High | hosts, snippets, layouts, settings, themes |
| Application/update artifacts | Medium | Critical | High | installers, metadata, signatures, dependency evidence |
| Privacy and diagnostics | High | High | Medium | crash data, connection metadata, retention choices |

## Attackers

- malicious local applications attempting to read files, credential-store data,
  IPC, clipboard, or process memory;
- malware or ransomware on the device;
- a thief with a stolen or unlocked device;
- compromised remote hosts returning malicious output or abusing forwarded
  authority and file operations;
- network attackers attempting interception, downgrade, DNS, proxy, or tunnel
  abuse;
- compromised dependencies, registries, build tools, or maintainers;
- insiders with access to build, release, signing, or support processes;
- attackers tampering with source, artifacts, update metadata, or diagnostics.

## Trust boundaries

1. React/webview to Rust core through allowlisted typed IPC.
2. Rust core to OS credential store, protected filesystem, and database.
3. Rust core to local shells, OpenSSH, askpass, editor, and helper processes.
4. Relio to remote SSH/SFTP/SCP, proxy, jump-host, and forwarding endpoints.
5. Release pipeline to signed installers and update metadata.
6. Relio to SSH configuration, theme records, and user-selected files.

## Attack scenarios and controls

| Scenario | Impact | Primary controls | Remaining risk |
| --- | --- | --- | --- |
| Compromised webview invokes privileged commands | Unauthorized local or remote action | Bundled assets, restrictive content policy, explicit Tauri capabilities, narrow IPC, core policy, trusted confirmation, no frontend secrets | An authorization defect in an allowed core operation remains critical |
| Local attacker copies the profile | Disclosure of hosts, history, settings, recordings | Encrypted SQLite/blobs/backups, random root key in OS secret store, native ACLs | An unlocked credential store or active process exposes decrypted data |
| Stolen unlocked device opens Relio | Unauthorized remote operations | OS lock awareness, credential re-authentication, keychain access controls, visible targets | Active OS-session authority remains usable |
| Host-key change is accepted casually | Interception and credential exposure | Strict known-host verification, fingerprint evidence, changed-key block, no silent replacement | A user can override a warning after explicit review |
| Weak SSH algorithm is negotiated | Downgrade or reduced confidentiality | Maintained defaults, app deny policy, per-host visible legacy exception | Some old hosts may require accepted residual risk |
| SSH configuration executes local code | Local execution or credential theft | Safe-subset parser, bounded includes, generated config, executable directives disabled | Parser defects or an incorrectly classified directive |
| Secret leaks through helper, IPC, diagnostics, clipboard, or process metadata | Credential compromise | Purpose-bound leases, one-time protected askpass channel, no args/env/frontend secret bytes, previewed diagnostics, clipboard opt-in | Same-user malware and imperfect redaction |
| Terminal output exploits renderer integration | UI compromise, spoofing, or unsafe action | Maintained parser, restrictive URI/clipboard policy, bounded stream, no operation authorization from output | Parser/runtime defects and social engineering |
| Snippet or history reuse submits hidden or malformed input | Unintended command execution | Single-line/control-character validation, complete preview, target/identity display, insertion without synthetic Enter | User can still submit a harmful reviewed line |
| Unsafe path or command composition is exploited | Unauthorized write or execution | Structured arguments, path revalidation, no shell concatenation, explicit target/scope | A legitimate user can still approve destructive work |
| Interrupted or hostile SFTP/SCP transfer corrupts or overwrites data | Data loss or unintended write | Structured paths, conflict and overwrite review, temporary destination, verification, legacy protocol unsupported | Remote filesystem semantics may prevent atomicity |
| Port forwarding exposes a service | Network exposure | Loopback default, broad-bind warning, visible endpoints, owned lifecycle | User can intentionally bind broadly |
| Remote output imitates trusted UI | Social engineering | Output remains in untrusted surfaces, reserved safety UI, explicit core-owned confirmation | Users may manually follow malicious text |
| Malformed or tampered theme state spoofs trusted UI | Social engineering | Bounded schema, data-only themes, invariant safety chrome, safe fallback | Convincing labels or colors can still mislead |
| Terminal output containing a password is retained | Secret disclosure | Retention off by default, warnings, encryption, quotas, deletion, best-effort redaction | Perfect secret detection is impossible |
| Update or build pipeline is compromised | Broad code execution across users | Protected build/signing, pinned dependencies, signed artifacts/metadata, provenance, SBOM, staged rollout, rollback | A validly signed malicious release remains catastrophic |
| Old valid update is replayed or client is frozen | Known vulnerability remains installed | Expiring metadata, highest-seen sequence/version, target/channel binding, freshness failure | Compromise of the initial signing role has limited survivability |
| Database key is lost or migration is interrupted | Permanent loss or unavailable profile | Credential-store error handling, encrypted recovery backup, transactional migration, rollback, no silent key replacement | Data is unrecoverable without a usable key/backup |

## Security objectives

- Secret bytes never enter ordinary application data or frontend state.
- Sensitive operations bind explicit user intent to target, identity, path,
  port, and operation.
- Untrusted terminal, remote, theme, and imported content cannot directly invoke
  privileged core behavior.
- Host identity and TLS certificates are verified rather than inferred from
  convenience.
- Local retained data uses platform controls and authenticated encryption.
- Every queue, process, transfer, cache, and recording has an owner and bound.
- Application behavior comes from the reviewed signed product, reducing
  runtime code-loading and distribution attack surface.
- Failures are visible, diagnosable, cancellable, and fail closed where
  authenticity or authority is uncertain.

## Accepted residual risks

- A fully compromised OS can observe input, memory, windows, clipboard, and
  network activity.
- Recordings and logs can contain secrets that automated redaction misses.
- User-approved commands and file writes can still cause damage.
- Remote filesystems may lack atomic rename or reliable metadata.
- Availability depends on OS services, external executables, remote hosts, and
  network paths.
- Cross-platform security parity requires platform-specific testing and may not
  be perfect.

Residual risks are reviewed before stable release and appear in release notes
when user action or platform configuration is required.

## Out-of-scope guarantees

Relio cannot guarantee:

- secrecy or integrity on a fully compromised or unlocked operating system;
- that a remote system or user-approved command is benign;
- physical deletion from SSDs, snapshots, backups, or remote systems;
- availability of credential store, network, remote host, webview, or OpenSSH;
- safety of an external executable deliberately launched by the user;
- recovery after all root and recovery keys are lost.
