# Network Security

## General rule

Network access is explicit, authenticated, observable, and validated. Relio
must not disable certificate or host-key verification to “make a connection
work.”

Every core destination belongs to the
[privacy egress registry](privacy.md#network-egress-registry). The application
performs no network request during startup.

## TLS and certificates

For the update service:

- use the platform trust store or an audited trust configuration;
- require certificate and hostname validation;
- prefer modern TLS versions and disable obsolete protocols and algorithms;
- do not provide a global “accept invalid certificates” switch;
- make test/development exceptions scoped, visible, and unavailable in stable builds where possible;
- validate redirects and prevent credential leakage across origins.

Certificate pinning is not a default substitute for normal trust validation; if a provider requires pinning, document rotation and failure recovery before shipping it.

## Proxies

Proxy configuration is security-sensitive. Relio supports platform proxy
settings and explicit authenticated network proxies without embedding
credentials in URLs or logs. Executable proxy commands are unsupported in v1.
A proxy must not silently downgrade TLS or SSH verification.

## SSH and tunneling

SSH host-key verification, cipher policy, jump-host display, and agent-forwarding consent are defined in [SSH security](ssh.md). Tunnels show local bind, remote destination, transport host, direction, state, and lifecycle. Loopback is the default; broad binds require an explicit warning and confirmation.

## MITM and downgrade protection

- reject invalid certificates and changed SSH host keys;
- keep protocol verification independent from UI convenience settings;
- do not retry through an alternate route with weaker security;
- surface algorithm and verification state in connection details;
- keep a reviewable record of user-approved exceptions with scope and expiry where possible.

## Connection logging

Connection diagnostics may record endpoint metadata, timing, protocol, algorithm, verification state, proxy/jump path, and error class. They must not record secret values, authorization headers, private keys, passwords, or raw payloads by default.

## Egress boundaries

User-initiated remote operations may connect only to the host, jump host, proxy,
file-transfer endpoint, or forwarding endpoint visible in the operation
review. The core validates destination and port after configuration expansion
and redirect/proxy resolution. SSH configuration, theme data, and terminal
output cannot create connections.

The stable application performs no background product analytics and has no
built-in diagnostic upload.

## Update network

Update origins are compiled into the trusted application configuration.
Redirects remain inside the allowlist. TLS is followed by signed metadata,
artifact signature, digest/length, target, channel, version, and OS-signature
verification as defined in [update security](updates.md).
