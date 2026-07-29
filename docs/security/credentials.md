# Credential Security

## Design goals

Relio must make ordinary credential handling safe without inventing a private credential database. Credentials belong in the operating system’s protected secret facility or an existing agent. Relio stores opaque references, metadata, and user intent—not secret material.

## Supported secure stores

The first-class integration targets are:

- Windows Credential Manager / Windows credential APIs;
- macOS Keychain Services;
- Linux Secret Service-compatible keychains such as GNOME Keyring or KDE Wallet.

If a secure store is unavailable, Relio should explain the limitation and avoid silently falling back to plaintext. A user may use an external agent or a deliberate, clearly marked session-only secret where supported; that exception must not become persistent storage.

## Data classification

| Data | Storage rule |
| --- | --- |
| Passwords and passphrases | OS keychain item; never SQLite, config, logs, URLs, or command arguments |
| Private keys | Reference to an OS keychain item or external key file/agent; never copied into ordinary app data |
| Access tokens and API keys | OS keychain item; scoped per provider and purpose |
| Certificates | Public certificate may be metadata; private key follows private-key rules |
| Host metadata | SQLite/config after sensitivity review; redact on export where needed |
| Fingerprints and known-host records | Integrity-protected local data with explicit verification history |
| Secret handles | Non-sensitive opaque identifiers, scoped to the owning profile/provider |

Relio must never store plaintext passwords, plaintext private keys, or plaintext tokens in SQLite, JSON, logs, crash reports, telemetry, plugin messages, URLs, command-line arguments, or environment variables unless a specific OS API requires a short-lived in-memory handoff.

## Credential lifecycle

1. The user selects a credential purpose and target host/provider.
2. Relio requests or generates a keychain item through the platform adapter.
3. The core receives an opaque handle or a short-lived secret result only at the operation boundary.
4. The transport uses the credential through a protected API, agent, or carefully supervised process.
5. Relio clears references and transient buffers as soon as the operation allows.
6. Removal revokes the app’s reference and requests deletion from the OS store where the user confirms.

The UI should show which credential source is being used without exposing secret values.

## Private-key import

Import must be user-selected and explicit. Relio should:

- accept only a path selected through the platform picker or an existing agent;
- verify file type and permissions before reading;
- parse and validate the key using a maintained cryptographic implementation or delegate to the platform/agent;
- never copy the private key into the workspace, SQLite, plugin storage, logs, or crash reports;
- store a protected keychain item or keep an external-file reference according to the user’s choice;
- warn when an external file is readable by other users or stored on an unencrypted volume;
- support cancellation and cleanup of temporary buffers.

Relio cannot guarantee physical deletion of a source file on SSDs or snapshots. When a temporary file is unavoidable, use restrictive permissions, minimize lifetime, and document the limitation.

## Agent integration

Agent use is preferred when it avoids exposing private-key material to the application. The UI must identify the selected agent and key identity. Agent forwarding to a remote host is disabled by default, requires an explicit per-session decision, and shows that the remote host may request signatures using the forwarded agent.

## Re-authentication and locking

Sensitive actions may require OS keychain re-authentication or an application lock:

- first use of a credential in a session;
- adding or changing a credential;
- enabling agent forwarding;
- exporting host/workspace data;
- binding a tunnel beyond loopback;
- executing a generated or destructive operation.

Relio should notice OS session lock where the platform permits it and pause or invalidate sensitive operations according to a documented policy.

## Diagnostics

Credential errors are classified without echoing secret values. Support bundles contain credential metadata only after a preview and explicit consent. A diagnostic must never say more than necessary to distinguish unavailable keychain, denied access, invalid key, authentication failure, or user cancellation.
