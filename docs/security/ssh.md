# SSH Security

## Security baseline

SSH connections must fail closed on identity and cryptographic verification errors. Relio uses the platform’s established SSH configuration and agent behavior where possible, but the application owns the consent flow, target display, error classification, and secure defaults.

## Key handling

- prefer an OS keychain, hardware-backed key, or agent;
- do not import private-key bytes into ordinary Relio storage;
- do not place passphrases in command lines or environment variables;
- show key identity and source before authentication;
- warn on private-key files with permissive access controls;
- distinguish a public key, private key, certificate, and agent identity in the UI;
- support rotation and removal without leaving stale references in workspace exports.

See [credential security](credentials.md) for the full lifecycle and import rules.

## Agent forwarding

Agent forwarding is a high-risk delegation: a remote host may request signatures from a forwarded agent. It is disabled by default, enabled per session or host profile, and must show:

- the remote target and jump chain;
- which local agent is being forwarded;
- that the remote host can request signatures while forwarding is active;
- how to disable it and whether reconnect will re-enable it.

Forwarding must not be enabled implicitly by importing an SSH config file. Prefer constrained alternatives such as jump hosts or per-host keys when they satisfy the workflow.

## Host-key verification

- use the user’s known-hosts files through a controlled adapter;
- display the algorithm, fingerprint, host, port, and verification source;
- require explicit trust for a first-seen key;
- block and explain changed or revoked keys;
- do not silently delete, replace, or rewrite known-hosts entries;
- preserve the original line and verification history where the platform permits it;
- make hashed host entries work without exposing unrelated host data.

Fingerprint display should use a modern, unambiguous representation and offer a copy action that does not put the private key or password on the clipboard.

## Algorithms and weak-cipher prevention

Relio maintains a reviewed cryptographic policy rather than accepting whatever a remote endpoint offers. Modern secure host-key algorithms, key exchange methods, ciphers, and MACs are enabled according to the underlying provider’s supported security baseline. Deprecated or weak algorithms are disabled by default.

Legacy exceptions require an explicit per-host override, a warning, a reason, and a visible review/expiry path. They must never lower the policy globally or silently apply to other hosts. The exact allowlist belongs in versioned implementation configuration and must be updated through a security review.

## SSH configuration and proxies

Imported SSH configuration is untrusted input. Relio parses it without shell evaluation, displays effective values, and warns when a `ProxyCommand`, local command, dynamic forwarding, or environment hook requires execution. User-provided commands are never concatenated with unvalidated values.

Proxy and jump-host chains show every hop and identity. A failure in one hop must not cause Relio to retry a different, less secure route automatically.

## SFTP and forwarding

SFTP writes require target, path, permissions, overwrite behavior, and conflict state to be visible. Port forwarding defaults to loopback and displays the local bind address, remote destination, transport host, and lifecycle. Binding beyond loopback requires explicit consent and may require re-authentication.

## Logging

Connection logs may include timestamps, host aliases, ports, algorithms, fingerprints, and failure classes. They must not include passwords, private keys, session tokens, full command arguments containing secrets, or raw remote output unless the user explicitly records a session and accepts the risk.
