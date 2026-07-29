# SSH Security

## Security baseline

SSH connections must fail closed on identity and cryptographic verification errors. Relio uses the platform’s established SSH configuration and agent behavior where possible, but the application owns the consent flow, target display, error classification, and secure defaults.

The first provider is a declared, tested range of system OpenSSH versions. An
unknown version or unavailable executable produces capability diagnosis rather
than optimistic parsing.

## Key handling

- prefer an OS keychain, hardware-backed key, or agent;
- do not import private-key bytes into ordinary Relio storage;
- do not place passphrases in command lines or environment variables;
- show key identity and source before authentication;
- warn on private-key files with permissive access controls;
- distinguish a public key, private key, certificate, and agent identity in the UI;
- support rotation and removal without leaving stale references in workspace exports.
- pass interactive answers only through the authenticated one-time askpass
  channel defined in [secret management](secrets.md).

See [credential security](credentials.md) for the full lifecycle and key-file
registration rules.

## Agent forwarding

Agent forwarding is a high-risk delegation: a remote host may request signatures from a forwarded agent. It is disabled by default, enabled per session or host profile, and must show:

- the remote target and jump chain;
- which local agent is being forwarded;
- that the remote host can request signatures while forwarding is active;
- how to disable it and whether reconnect will re-enable it.

Forwarding must not be enabled implicitly by importing an SSH config file. Prefer constrained alternatives such as jump hosts or per-host keys when they satisfy the workflow.

## Host-key verification

- use a Relio-managed known-hosts store for Relio profiles and approved user
  known-hosts files as controlled read-only sources;
- display the algorithm, fingerprint, host, port, and verification source;
- require explicit trust for a first-seen key;
- block and explain changed or revoked keys;
- do not silently delete, replace, or rewrite known-hosts entries;
- preserve the original line and verification history where the platform permits it;
- make hashed host entries work without exposing unrelated host data.
- never use `ssh-keyscan` output alone as proof of identity.

Fingerprint display should use a modern, unambiguous representation and offer a copy action that does not put the private key or password on the clipboard.

## Algorithms and weak-cipher prevention

Relio requires a supported OpenSSH baseline and does not weaken its maintained
defaults globally. A small app-level deny policy may block algorithms that must
not be re-enabled even if configuration requests them. Exact names and minimum
provider versions live in versioned security configuration and require
compatibility tests.

Legacy exceptions require an explicit per-host override, a warning, a reason, and a visible review/expiry path. They must never lower the policy globally or silently apply to other hosts. The exact allowlist belongs in versioned implementation configuration and must be updated through a security review.

## SSH configuration and proxies

Imported SSH configuration is untrusted input. Relio parses a documented safe
subset without shell evaluation, limits include traversal/depth/size, displays
effective values, and generates a protected minimal config for OpenSSH.

`LocalCommand`, `PermitLocalCommand`, `Match exec`, unknown directives, and
environment hooks are disabled in v1. `ProxyCommand` is unsupported; use
`ProxyJump` for supported jump-host workflows. User-provided values are never
concatenated into a shell string.

Proxy and jump-host chains show every hop and identity. A failure in one hop must not cause Relio to retry a different, less secure route automatically.

## SFTP, SCP, and forwarding

SFTP and SCP operations show source, destination host, exact remote path,
direction, overwrite behavior, and available integrity evidence. Prefer SFTP
semantics. The legacy SCP protocol is unsupported in v1 because remote-shell
path interpretation and wildcard behavior can make the peer influence file
selection. If Relio cannot prove that the selected `scp` executable uses SFTP
semantics, it does not offer that operation; the user can use the SFTP transfer
workflow instead.

Port forwarding defaults to loopback and displays the local bind address,
remote destination, transport host, and lifecycle. Binding beyond loopback
requires explicit consent and may require re-authentication.

Control sockets and helper endpoints live in a randomized, user-only runtime
directory. Relio stops only processes/listeners it owns and never identifies a
process to kill solely by port number.

## Logging

Connection logs may include timestamps, host aliases, ports, algorithms, fingerprints, and failure classes. They must not include passwords, private keys, session tokens, full command arguments containing secrets, or raw remote output unless the user explicitly records a session and accepts the risk.

## Required negative tests

- hostile host aliases, usernames, options, and local/remote paths remain
  structured arguments;
- legacy SCP is refused on every host;
- wildcard, leading-dash, newline, control-character, and traversal-like names
  cannot become command options or unexpected selections;
- interrupted transfers leave no silently accepted partial destination;
- overwrite, symlink, host-key, jump-host, and helper failures remain visible
  and fail closed.
