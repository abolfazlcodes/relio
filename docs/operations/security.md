# Security Considerations

This document remains the operations-level summary. The canonical security-first architecture for Relio is in [`docs/security/`](../security/README.md).

## Threat model

The application handles valuable assets:

- SSH host identities and connection metadata;
- passwords, private keys, agents, tokens, and certificates;
- terminal input and output, which may contain secrets;
- local files and remote file contents;
- port-forward listeners that can expose services;
- plugins and downloaded packages;
- optional AI context and provider credentials.

Threats include a malicious plugin, compromised package or update, webview injection, accidental secret logging, host-key spoofing, command injection through unsafe argument composition, leaked local files, and unexpected network activity.

## Secure defaults

- core operation works offline;
- credentials use the OS credential store or agent integration;
- secrets never appear in ordinary logs, settings exports, crash reports, or plugin messages;
- host-key changes require explicit review;
- local forwarding binds to loopback by default;
- destructive or externally visible operations show target and scope;
- AI output is untrusted text until the user reviews it;
- plugins are disabled or isolated when their runtime fails;
- updates are verified before installation;
- telemetry is opt-in and documented.

## Trust boundaries

```text
untrusted remote system
        ↕ network transport
transport adapter
        ↕ typed application contract
Rust application core
        ↕ capability-scoped IPC
plugin host / plugin
        ↕ constrained rendering contract
React workbench
```

The webview is not a secret store. The frontend receives only the minimum data needed to render a view. The plugin host is not trusted with unrestricted core access. Remote output is data and must not be interpreted as an instruction to the application without an explicit protocol.

## Secret handling

- represent credentials by opaque references;
- avoid accepting secrets in command-line arguments where process inspection can expose them;
- redact known secret values and patterns from diagnostics, while warning that redaction is not perfect;
- clear sensitive buffers promptly where the language and platform make that meaningful;
- never include session recordings in support bundles by default;
- make credential deletion and rotation discoverable.

## Command and filesystem safety

Arguments are passed as structured process arguments, not concatenated shell strings. User-selected paths are normalized and revalidated at the operation boundary. Operations that write remote files, bind network ports, or execute generated commands require an explicit final action.

## Plugin and supply-chain safety

Plugin packages need publisher identity, version, compatibility, requested capabilities, integrity information, and license metadata. A future marketplace should support signatures, but signature verification must not be the only defense: isolation and least privilege remain required.

## Privacy

Local data locations and retention are documented. The app should offer a diagnostic export that previews included files and redacts secrets. Optional sync and AI providers must state what leaves the device, for what purpose, and how the user disables it.

## Security review triggers

Require a security review when changing authentication, host-key handling, secret storage, plugin capabilities, update verification, remote file writes, port binding, AI context flow, or telemetry.
