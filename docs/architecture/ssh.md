# SSH Architecture

## Goals

Provide reliable, understandable SSH workflows while preserving users’ existing configuration and security expectations. SSH is a transport, not the definition of a workspace.

## Transport contract

The application-level contract should cover:

- connect, authenticate, and disconnect;
- interactive channel with PTY dimensions;
- non-interactive command execution;
- SFTP-like file operations;
- local, remote, and dynamic forwarding;
- host-key status and verification result;
- jump host / proxy chain metadata;
- cancellation, timeout, reconnect, and typed errors.

Protocol-specific adapters implement this contract. A UI view should not know whether the active adapter uses an OpenSSH process, a native library, or a future plugin.

## First provider strategy

Start with an OpenSSH-compatible adapter where available. Read standard SSH configuration and use platform credential helpers and agents where possible. On systems without the required executable, show a clear capability diagnosis and leave room for a native provider.

This approach favors compatibility with real-world configurations in the first useful milestone. The adapter must still own process lifecycle, argument construction, environment filtering, stderr classification, and cancellation rather than building shell command strings from user input.

## Host model

A host record contains connection metadata such as alias, address, port, username, groups, tags, jump-host references, terminal profile, and secret handles. It does not contain plaintext passwords or private key contents. Host records are local-first and exportable with secrets redacted.

## Authentication and host keys

- prefer the user’s agent or OS credential integration;
- never log passwords, private keys, or access tokens;
- verify host keys against known hosts and show changed-key warnings prominently;
- never silently accept a changed host key;
- make first-use trust decisions explicit and explain the fingerprint;
- do not allow a plugin to read host keys or secrets unless a future capability explicitly permits it.

## SFTP and remote editing

The file browser uses the same provider session where possible. Remote editing follows an explicit temporary-local-file workflow:

1. download a versioned snapshot;
2. edit locally in the application editor or configured editor;
3. compare remote metadata before upload;
4. show a conflict decision if the remote file changed;
5. upload only after user confirmation;
6. preserve an audit-friendly operation result.

## Port forwarding

Forwarding is modeled as a durable operation with source, destination, transport host, direction, bind address, state, and lifecycle timestamps. The visual manager must show what is listening locally and where traffic will go. Binding to all interfaces requires an explicit warning.

## Failure behavior

Errors should distinguish authentication failure, host-key failure, network timeout, proxy/jump failure, remote command failure, local executable absence, and user cancellation. Reconnect must be a deliberate policy, not an automatic loop that can create duplicate tunnels or unexpected commands.
