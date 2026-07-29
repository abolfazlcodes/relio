# Security Considerations

This document remains the operations-level summary. The canonical security-first architecture for Relio is in [`docs/security/`](../security/README.md).

## Threat model

The application handles valuable assets:

- SSH host identities and connection metadata;
- passwords, private keys, agents, passphrases, and certificates;
- terminal input and output, which may contain secrets;
- local files and remote file contents;
- port-forward listeners that can expose services;
- application, dependency, and update artifacts.

Threats include a compromised dependency or update, webview injection,
accidental secret logging, host-key spoofing, command injection through unsafe
argument composition, malicious remote content, leaked local files, and
unexpected network activity.

## Secure defaults

- core operation works offline;
- credentials use the OS credential store or agent integration;
- the local profile database and retained high-sensitivity content are encrypted
  at rest with keys rooted in the OS secret store;
- secrets never appear in ordinary logs, settings exports, crash reports, or
  frontend IPC;
- host-key changes require explicit review;
- local forwarding binds to loopback by default;
- destructive or externally visible operations show target and scope;
- all application behavior is compiled, reviewed, and signed with Relio;
- themes and SSH configuration are bounded data and cannot execute scripts or load remote
  application content;
- updates are verified before installation;
- the initial stable release has no background product analytics; any future
  telemetry requires a separate opt-in privacy design.

## Trust boundaries

```text
untrusted remote system
        ↕ network transport
transport adapter
        ↕ typed application contract
Rust application core
        ↕ allowlisted typed IPC
React workbench
```

The webview is not a secret store. The frontend receives only the minimum data
needed to render a view. Remote output is data and must not be interpreted as
an instruction to the application without an explicit protocol.

## Secret handling

- represent credentials by opaque references;
- resolve secret bytes only through purpose-bound, short-lived core leases;
- avoid accepting secrets in command-line arguments where process inspection can expose them;
- redact known secret values and patterns from diagnostics, while warning that redaction is not perfect;
- clear sensitive buffers promptly where the language and platform make that meaningful;
- never include session recordings in support bundles by default;
- make credential deletion and rotation discoverable.

## Command and filesystem safety

Arguments are passed as structured process arguments, not concatenated shell strings. User-selected paths are normalized and revalidated at the operation boundary. Operations that write remote files, bind network ports, or execute generated commands require an explicit final action.

## Dependency and update safety

Dependencies are pinned and reviewed for maintenance, advisories, licenses,
build scripts, native code, and platform impact. Release artifacts and update
metadata are signed, target-specific, expiry-bounded, and rollback-aware.
Relio uses one authoritative update origin and does not load executable content
from imported data or remote systems.

## Privacy

Local data locations and retention are documented. The app should offer a
diagnostic export that previews included files and redacts secrets. Remote
connections occur only for user-initiated operations, update checks follow the
documented policy, and v1 has no built-in diagnostic upload.

The initial stable release has no background product analytics. See
[privacy principles](../security/privacy.md).

## Security review triggers

Require a security review when changing authentication, host-key handling,
secret storage/leases, encryption, frontend IPC exposure, update verification,
remote file writes, port binding, command execution, recording, diagnostic
collection, or product analytics.
