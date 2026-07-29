# Credential Storage and Security

## Design goals

Relio must make ordinary credential handling safe without inventing a private
plaintext credential database. Credentials belong in the operating system's
protected secret facility, a hardware-backed identity, an existing agent, or a
user-controlled external key file. Relio stores opaque references, encrypted
metadata, and user intent—not secret material in ordinary records.

The acquisition, lease, handoff, and memory rules are in
[secret management](secrets.md).

## Supported secure stores

The first-class integration targets are:

- Windows Credential Manager / Windows credential APIs;
- macOS Keychain Services;
- Linux Secret Service-compatible keychains such as GNOME Keyring or KDE Wallet.

The concrete API and library adapter must be validated on every Tier 1 platform;
the product does not assume that all “keyring” abstractions have equivalent
unlock, access-control, size, or deletion behavior.

If a secure store is unavailable, Relio keeps the encrypted profile closed and
never silently falls back to plaintext. A user may use an external agent or a
deliberate, clearly marked session-only secret in a temporary no-persistence
mode; that exception must not become persistent storage.

## Data classification

| Data | Storage rule |
| --- | --- |
| Passwords and passphrases | OS keychain item; never SQLite, config, logs, URLs, or command arguments |
| Private keys | Prefer hardware/agent or reference a user-controlled external key file; v1 never copies key bytes into the profile |
| Certificates | Public certificate may be metadata; private key follows private-key rules |
| Host metadata | SQLite/config after sensitivity review; redact on export where needed |
| Fingerprints and known-host records | Integrity-protected local data with explicit verification history |
| Secret handles | Sensitive metadata; random opaque identifiers scoped to the owning profile/provider |
| Profile root key | OS secret store/protected data API; never user-derived or exported in an ordinary backup |

Secret handles are not authentication material, but they reveal relationships
and are treated as sensitive metadata. Relio must never store plaintext
passwords, private keys, passphrases, or root keys in SQLite, JSON, logs, crash
reports, product analytics, frontend IPC, URLs, command-line arguments, or ordinary
environment variables.

## Credential lifecycle

1. The user selects a credential purpose and target host.
2. Relio requests or generates a keychain item through the platform adapter.
3. The core receives an opaque handle; only the secret service can resolve a
   short-lived, purpose-bound lease at the operation boundary.
4. The transport uses the credential through a protected API, agent, or
   authenticated one-time helper channel.
5. Relio clears references and transient buffers as soon as the operation allows.
6. Removal revokes the app’s reference and requests deletion from the OS store where the user confirms.

The UI should show which credential source is being used without exposing secret values.

## Private-key registration

Registration must be user-selected and explicit. V1 registers an existing key
file or agent identity and never imports key bytes. Relio must:

- accept only a path selected through the platform picker or an existing agent;
- resolve and verify that the selected path is a regular file with acceptable
  native access controls;
- delegate key parsing and use to the supported OpenSSH process or agent rather
  than reading private-key bytes during registration;
- never copy the private key into a workspace, SQLite, logs, or crash reports;
- store the path reference only in the encrypted profile and warn that a
  same-user process may observe paths passed to OpenSSH;
- warn when an external file is readable by other users or stored on an unencrypted volume;
- fail closed if the file is replaced with a non-regular or more broadly
  accessible object before use.

Relio does not modify or delete the user-controlled key file. A moved, replaced,
or unavailable file produces a visible unresolved credential reference rather
than a fallback credential.

## Agent integration

Agent use is preferred when it avoids exposing private-key material to the application. The UI must identify the selected agent and key identity. Agent forwarding to a remote host is disabled by default, requires an explicit per-session decision, and shows that the remote host may request signatures using the forwarded agent.

## Re-authentication and locking

Sensitive actions may require OS keychain re-authentication:

- first use of a credential in a session;
- adding or changing a credential;
- enabling agent forwarding;
- exporting host/workspace data;
- binding a tunnel beyond loopback;
- executing a destructive command or operation.

Relio should notice OS session lock where the platform permits it, clear secret
leases, and pause or terminate credential-dependent pending operations according
to a documented provider policy.

An application lock is a privacy/convenience control, not a boundary against
malware or an attacker controlling the same unlocked OS account. It must not be
described as equivalent to OS re-authentication.

## Diagnostics

Credential errors are classified without echoing secret values. Support bundles contain credential metadata only after a preview and explicit consent. A diagnostic must never say more than necessary to distinguish unavailable keychain, denied access, invalid key, authentication failure, or user cancellation.
